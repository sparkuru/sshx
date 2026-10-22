use std::io::{BufRead, Cursor, Read, Write};

use sha2::{Digest, Sha256};

pub const PNG_MAX: usize = 16 * 1024 * 1024;
pub const RAW_MAX: usize = 64 * 1024 * 1024;
pub const DIM_MAX: u32 = 8192;
pub const PIXELS_MAX: usize = 16_777_216;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn fail(message: &str) -> Box<dyn std::error::Error> {
    std::io::Error::other(message).into()
}

pub fn layout(width: u32, height: u32) -> Result<usize> {
    if width == 0 || height == 0 || width > DIM_MAX || height > DIM_MAX {
        return Err(fail("LIMIT_EXCEEDED"));
    }
    let pixels = (width as usize)
        .checked_mul(height as usize)
        .filter(|pixels| *pixels <= PIXELS_MAX)
        .ok_or_else(|| fail("LIMIT_EXCEEDED"))?;
    pixels
        .checked_mul(4)
        .filter(|bytes| *bytes <= RAW_MAX)
        .ok_or_else(|| fail("LIMIT_EXCEEDED"))
}

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

struct BoundedWriter(Vec<u8>);
impl Write for BoundedWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let end = self
            .0
            .len()
            .checked_add(bytes.len())
            .ok_or_else(|| std::io::Error::other("LIMIT_EXCEEDED"))?;
        if end > PNG_MAX {
            return Err(std::io::Error::other("LIMIT_EXCEEDED"));
        }
        // Exact growth prevents Vec's geometric capacity from exceeding the encoded cap.
        self.0
            .try_reserve_exact(bytes.len())
            .map_err(std::io::Error::other)?;
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn encode(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>> {
    if layout(width, height)? != rgba.len() {
        return Err(fail("CORRUPT_IMAGE"));
    }
    let mut output = BoundedWriter(Vec::new());
    {
        let mut encoder = png::Encoder::new(&mut output, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(rgba)?;
    }
    preflight(&output.0, Some((width, height)))?;
    Ok(output.0)
}

/// Reject all metadata and validate the complete static RGBA8 profile before decoding.
/// This scan borrows encoded bytes and never allocates proportional to a PNG claim.
pub fn preflight(bytes: &[u8], expected: Option<(u32, u32)>) -> Result<(u32, u32, usize)> {
    if bytes.len() > PNG_MAX {
        return Err(fail("LIMIT_EXCEEDED"));
    }
    if bytes.len() < 33 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err(fail("CORRUPT_IMAGE"));
    }
    let mut offset = 8usize;
    let mut dimensions = None;
    let mut idat = false;
    let mut srgb = false;
    let mut end = false;
    while offset < bytes.len() {
        let header_end = offset
            .checked_add(8)
            .filter(|n| *n <= bytes.len())
            .ok_or_else(|| fail("CORRUPT_IMAGE"))?;
        let length = u32::from_be_bytes(bytes[offset..offset + 4].try_into()?) as usize;
        let chunk_end = header_end
            .checked_add(length)
            .and_then(|n| n.checked_add(4))
            .filter(|n| *n <= bytes.len())
            .ok_or_else(|| fail("CORRUPT_IMAGE"))?;
        let kind = &bytes[offset + 4..header_end];
        let data = &bytes[header_end..chunk_end - 4];
        let crc = u32::from_be_bytes(bytes[chunk_end - 4..chunk_end].try_into()?);
        if crc32fast::hash(&bytes[offset + 4..chunk_end - 4]) != crc {
            return Err(fail("CORRUPT_IMAGE"));
        }
        match kind {
            b"IHDR" if offset == 8 && data.len() == 13 => {
                let width = u32::from_be_bytes(data[..4].try_into()?);
                let height = u32::from_be_bytes(data[4..8].try_into()?);
                let raw = layout(width, height)?;
                if data[8..] != [8, 6, 0, 0, 0]
                    || expected.is_some_and(|value| value != (width, height))
                {
                    return Err(fail("CORRUPT_IMAGE"));
                }
                dimensions = Some((width, height, raw));
            }
            b"sRGB"
                if dimensions.is_some() && !idat && !srgb && data.len() == 1 && data[0] <= 3 =>
            {
                srgb = true;
            }
            b"IDAT" if dimensions.is_some() && !end => {
                idat = true;
            }
            b"IEND" if idat && data.is_empty() && chunk_end == bytes.len() => {
                end = true;
            }
            _ => return Err(fail("CORRUPT_IMAGE")),
        }
        offset = chunk_end;
    }
    if !end {
        return Err(fail("CORRUPT_IMAGE"));
    }
    dimensions.ok_or_else(|| fail("CORRUPT_IMAGE"))
}

pub fn decode(bytes: &[u8], expected: Option<(u32, u32)>) -> Result<(u32, u32, Vec<u8>)> {
    let (width, height, raw) = preflight(bytes, expected)?;
    validate_zlib_length(bytes, width, height)?;
    // The profile excludes text, ICC, APNG and interlace allocations. The library's
    // remaining row/decompression allocations are additionally limited and measured.
    let decoder = png::Decoder::new_with_limits(
        Cursor::new(bytes),
        png::Limits {
            bytes: RAW_MAX + 8 * 1024 * 1024,
        },
    );
    let mut reader = decoder.read_info()?;
    if reader.output_buffer_size() != Some(raw) {
        return Err(fail("CORRUPT_IMAGE"));
    }
    let mut output = Vec::new();
    output.try_reserve_exact(raw)?;
    output.resize(raw, 0);
    let info = reader.next_frame(&mut output)?;
    if info.buffer_size() != raw || info.width != width || info.height != height {
        return Err(fail("CORRUPT_IMAGE"));
    }
    reader.finish()?;
    Ok((width, height, output))
}

struct IdatReader<'a> {
    png: &'a [u8],
    offset: usize,
    current: &'a [u8],
}
impl BufRead for IdatReader<'_> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        while self.current.is_empty() && self.offset < self.png.len() {
            let start = self.offset;
            let length = u32::from_be_bytes(
                self.png[start..start + 4]
                    .try_into()
                    .expect("preflighted length"),
            ) as usize;
            self.offset = start + 12 + length;
            if &self.png[start + 4..start + 8] == b"IDAT" {
                self.current = &self.png[start + 8..start + 8 + length];
            }
        }
        Ok(self.current)
    }
    fn consume(&mut self, amount: usize) {
        self.current = &self.current[amount..];
    }
}
impl Read for IdatReader<'_> {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        let available = self.fill_buf()?;
        let length = available.len().min(output.len());
        output[..length].copy_from_slice(&available[..length]);
        self.consume(length);
        Ok(length)
    }
}
/// png::Reader accepts surplus zlib output in this tested release. Independently
/// count the complete scanline stream with fixed scratch memory before decode.
fn validate_zlib_length(bytes: &[u8], width: u32, height: u32) -> Result<()> {
    let expected = (u64::from(width) * 4 + 1) * u64::from(height);
    let mut compressed_length = 0u64;
    let mut offset = 8;
    while offset < bytes.len() {
        let length = u32::from_be_bytes(bytes[offset..offset + 4].try_into()?) as usize;
        if &bytes[offset + 4..offset + 8] == b"IDAT" {
            compressed_length += length as u64;
        }
        offset += 12 + length;
    }
    let mut decoder = flate2::bufread::ZlibDecoder::new(IdatReader {
        png: bytes,
        offset: 8,
        current: &[],
    });
    let mut scratch = [0u8; 8192];
    let mut produced = 0u64;
    loop {
        let count = decoder.read(&mut scratch)?;
        if count == 0 {
            break;
        }
        produced += count as u64;
        if produced > expected {
            return Err(fail("CORRUPT_IMAGE"));
        }
    }
    if produced != expected || decoder.total_in() != compressed_length {
        return Err(fail("CORRUPT_IMAGE"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alpha_and_exact_sealed_bytes() {
        let rgba = [255, 0, 0, 0, 0, 255, 0, 127, 0, 0, 255, 255, 2, 3, 4, 9];
        let png = encode(2, 2, &rgba).unwrap();
        let sealed = digest(&png);
        assert_eq!(decode(&png, Some((2, 2))).unwrap().2, rgba);
        assert_eq!(digest(&png), sealed);
        assert!(decode(&png, Some((1, 2))).is_err());
    }
    #[test]
    fn reject_claims_before_output_allocation() {
        assert!(layout(u32::MAX, u32::MAX).is_err());
        assert!(layout(8192, 8192).is_err());
        assert!(layout(0, 1).is_err());
        assert_eq!(layout(4096, 4096).unwrap(), RAW_MAX);
        let mut png = encode(1, 1, &[0; 4]).unwrap();
        png[16..20].copy_from_slice(&u32::MAX.to_be_bytes());
        let crc = crc32fast::hash(&png[12..29]);
        png[29..33].copy_from_slice(&crc.to_be_bytes());
        assert_eq!(
            preflight(&png, None).unwrap_err().to_string(),
            "LIMIT_EXCEEDED"
        );
    }
    #[test]
    fn reject_crc_trailing_metadata_and_truncation() {
        let png = encode(1, 1, &[0; 4]).unwrap();
        for length in [0, 8, 32, png.len() - 1] {
            assert!(decode(&png[..length], None).is_err());
        }
        let mut bad = png.clone();
        bad[30] ^= 1;
        assert!(decode(&bad, None).is_err());
        let mut bad = png.clone();
        bad.push(0);
        assert!(decode(&bad, None).is_err());
        let mut bad = png.clone();
        let mut text = Vec::new();
        text.extend_from_slice(&1u32.to_be_bytes());
        text.extend_from_slice(b"tEXt");
        text.push(0);
        text.extend_from_slice(&crc32fast::hash(&text[4..]).to_be_bytes());
        bad.splice(33..33, text);
        assert!(decode(&bad, None).is_err());
    }
    #[test]
    fn reject_surplus_decompressed_pixels_and_streams() {
        let mut png = encode(32, 32, &[0; 4096]).unwrap();
        png[16..20].copy_from_slice(&1u32.to_be_bytes());
        png[20..24].copy_from_slice(&1u32.to_be_bytes());
        let crc = crc32fast::hash(&png[12..29]);
        png[29..33].copy_from_slice(&crc.to_be_bytes());
        assert!(preflight(&png, None).is_ok());
        assert_eq!(decode(&png, None).unwrap_err().to_string(), "CORRUPT_IMAGE");
    }
}
