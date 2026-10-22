//! Core-side independent validation: transfer never rewrites the sealed bytes.
use crate::Result;
use std::io::Cursor;
pub fn validate(bytes: &[u8]) -> Result<(u32, u32, String)> {
    if bytes.len() > 16 * 1024 * 1024 || bytes.len() < 33 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("CORRUPT_IMAGE".into());
    }
    let mut p = 8;
    let mut dims = None;
    let mut data_seen = false;
    let mut srgb = false;
    let mut ended = false;
    let mut zlib = flate2::Decompress::new(true);
    let mut stream_end = false;
    while p < bytes.len() {
        let h = p
            .checked_add(8)
            .filter(|n| *n <= bytes.len())
            .ok_or("CORRUPT_IMAGE")?;
        let n = u32::from_be_bytes(bytes[p..p + 4].try_into()?) as usize;
        let end = h
            .checked_add(n)
            .and_then(|n| n.checked_add(4))
            .filter(|n| *n <= bytes.len())
            .ok_or("CORRUPT_IMAGE")?;
        let data = &bytes[h..end - 4];
        let kind = &bytes[p + 4..h];
        if crc32fast::hash(&bytes[p + 4..end - 4])
            != u32::from_be_bytes(bytes[end - 4..end].try_into()?)
        {
            return Err("CORRUPT_IMAGE".into());
        }
        match kind {
            b"IHDR" if p == 8 && n == 13 => {
                let w = u32::from_be_bytes(data[..4].try_into()?);
                let h = u32::from_be_bytes(data[4..8].try_into()?);
                let pixels = (w as usize)
                    .checked_mul(h as usize)
                    .ok_or("LIMIT_EXCEEDED")?;
                if w == 0
                    || h == 0
                    || w > 8192
                    || h > 8192
                    || pixels > 16_777_216
                    || data[8..] != [8, 6, 0, 0, 0]
                {
                    return Err("LIMIT_EXCEEDED".into());
                }
                dims = Some((w, h, pixels * 4));
            }
            b"sRGB" if dims.is_some() && !data_seen && !srgb && n == 1 && data[0] <= 3 => {
                srgb = true
            }
            b"IDAT" if dims.is_some() && !ended => {
                data_seen = true;
                let (w, h, _) = dims.ok_or("CORRUPT_IMAGE")?;
                let expected = (h as u64) * ((w as u64) * 4 + 1);
                let mut offset = 0;
                let mut scratch = [0u8; 4096];
                loop {
                    if stream_end {
                        if offset != data.len() {
                            return Err("CORRUPT_IMAGE: trailing deflate data".into());
                        }
                        break;
                    }
                    let before_in = zlib.total_in();
                    let before_out = zlib.total_out();
                    let status = zlib.decompress(
                        &data[offset..],
                        &mut scratch,
                        flate2::FlushDecompress::None,
                    )?;
                    let consumed = (zlib.total_in() - before_in) as usize;
                    offset += consumed;
                    let produced = zlib.total_out() - before_out;
                    if zlib.total_out() > expected {
                        return Err("CORRUPT_IMAGE: decompression surplus".into());
                    }
                    if status == flate2::Status::StreamEnd {
                        stream_end = true;
                        continue;
                    }
                    if consumed == 0 && produced == 0 {
                        break;
                    }
                    if offset == data.len() && produced < scratch.len() as u64 {
                        break;
                    }
                }
            }
            b"IEND" if data_seen && n == 0 && end == bytes.len() => ended = true,
            _ => return Err("CORRUPT_IMAGE".into()),
        }
        p = end;
    }
    if !ended {
        return Err("CORRUPT_IMAGE".into());
    }
    let (w, h, size) = dims.ok_or("CORRUPT_IMAGE")?;
    if !stream_end || zlib.total_out() != (h as u64) * ((w as u64) * 4 + 1) {
        return Err("CORRUPT_IMAGE: deflate length".into());
    }
    let decoder = png::Decoder::new_with_limits(
        Cursor::new(bytes),
        png::Limits {
            bytes: 72 * 1024 * 1024,
        },
    );
    let mut reader = decoder.read_info()?;
    if reader.output_buffer_size() != Some(size) {
        return Err("CORRUPT_IMAGE".into());
    }
    let mut output = Vec::new();
    output.try_reserve_exact(size)?;
    output.resize(size, 0);
    let info = reader.next_frame(&mut output)?;
    if info.buffer_size() != size || info.width != w || info.height != h {
        return Err("CORRUPT_IMAGE".into());
    }
    reader.finish()?;
    Ok((w, h, crate::sha(bytes)))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_deflate_surplus_before_raw_output() {
        let mut bytes = Vec::new();
        {
            let mut e = png::Encoder::new(&mut bytes, 2, 2);
            e.set_color(png::ColorType::Rgba);
            e.set_depth(png::BitDepth::Eight);
            let mut w = e.write_header().unwrap();
            w.write_image_data(&[0; 16]).unwrap();
        }
        assert!(validate(&bytes).is_ok());
        bytes[16..20].copy_from_slice(&1u32.to_be_bytes());
        bytes[20..24].copy_from_slice(&1u32.to_be_bytes());
        let crc = crc32fast::hash(&bytes[12..29]);
        bytes[29..33].copy_from_slice(&crc.to_be_bytes());
        assert!(validate(&bytes)
            .unwrap_err()
            .to_string()
            .contains("decompression surplus"));
    }
}
