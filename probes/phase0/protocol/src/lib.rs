//! Bounded public wire subset for disposable Phase 0 processes.
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::fmt;
use std::io::{self, BufRead, Read, Write};

pub const HEADER_MAX: usize = 1024;
pub const BODY_MAX: usize = 262_144;
pub const JSON_DEPTH_MAX: usize = 32;
pub const INTEGER_MAX: u64 = 9_007_199_254_740_991;
pub const TERM_CHUNK_MAX: usize = 65_536;

#[derive(Debug)]
pub struct ProtocolError(pub String);
impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for ProtocolError {}
impl From<io::Error> for ProtocolError {
    fn from(value: io::Error) -> Self {
        Self(value.to_string())
    }
}

struct JsonSeed(usize);
impl<'de> DeserializeSeed<'de> for JsonSeed {
    type Value = Value;
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        if self.0 > JSON_DEPTH_MAX {
            return Err(de::Error::custom("JSON depth limit"));
        }
        d.deserialize_any(JsonVisitor(self.0))
    }
}
struct JsonVisitor(usize);
impl<'de> Visitor<'de> for JsonVisitor {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
        if v.unsigned_abs() > INTEGER_MAX {
            return Err(E::custom("integer range"));
        }
        Ok(Value::from(v))
    }
    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
        if v > INTEGER_MAX {
            return Err(E::custom("integer range"));
        }
        Ok(Value::from(v))
    }
    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| E::custom("nonfinite"))
    }
    fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value::String(v.into()))
    }
    fn visit_string<E: de::Error>(self, v: String) -> Result<Value, E> {
        Ok(Value::String(v))
    }
    fn visit_none<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut v = Vec::new();
        while let Some(x) = a.next_element_seed(JsonSeed(self.0 + 1))? {
            v.push(x);
        }
        Ok(Value::Array(v))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut v = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if v.contains_key(&k) {
                return Err(de::Error::custom("duplicate key"));
            }
            v.insert(k, a.next_value_seed(JsonSeed(self.0 + 1))?);
        }
        Ok(Value::Object(v))
    }
}

pub fn read_frame<R: BufRead>(reader: &mut R) -> Result<Option<Value>, ProtocolError> {
    let mut header = Vec::with_capacity(64);
    loop {
        let mut b = [0];
        if reader.read(&mut b)? == 0 {
            return if header.is_empty() {
                Ok(None)
            } else {
                Err(ProtocolError("truncated header".into()))
            };
        }
        header.push(b[0]);
        if header.len() > HEADER_MAX {
            return Err(ProtocolError("header limit".into()));
        }
        if !b[0].is_ascii()
            || (b[0] == b'\n' && (header.len() < 2 || header[header.len() - 2] != b'\r'))
        {
            return Err(ProtocolError("invalid header".into()));
        }
        if header.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    let line = std::str::from_utf8(&header[..header.len() - 4])
        .map_err(|e| ProtocolError(e.to_string()))?;
    let (name, length) = line
        .split_once(": ")
        .ok_or_else(|| ProtocolError("header syntax".into()))?;
    if !name.eq_ignore_ascii_case("Content-Length")
        || length.is_empty()
        || !length.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(ProtocolError("unsupported header".into()));
    }
    let n = length
        .parse::<usize>()
        .map_err(|_| ProtocolError("length overflow".into()))?;
    if n == 0 || n > BODY_MAX {
        return Err(ProtocolError("body limit".into()));
    }
    let mut body = vec![0; n];
    reader.read_exact(&mut body)?;
    if body.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(ProtocolError("BOM".into()));
    }
    let mut d = serde_json::Deserializer::from_slice(&body);
    let value = JsonSeed(0)
        .deserialize(&mut d)
        .map_err(|e| ProtocolError(e.to_string()))?;
    d.end().map_err(|e| ProtocolError(e.to_string()))?;
    if !value.is_object() {
        return Err(ProtocolError("single object required".into()));
    }
    Ok(Some(value))
}
pub fn write_frame<W: Write>(writer: &mut W, value: &Value) -> Result<(), ProtocolError> {
    let body = serde_json::to_vec(value).map_err(|e| ProtocolError(e.to_string()))?;
    if body.is_empty() || body.len() > BODY_MAX {
        return Err(ProtocolError("body limit".into()));
    }
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
    writer.write_all(&body)?;
    writer.flush()?;
    Ok(())
}

#[derive(Default, Debug)]
pub struct RequestCounter(u64);
impl RequestCounter {
    pub fn admit(&mut self, id: &str) -> Result<u64, ProtocolError> {
        if id.starts_with('0') || id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
            return Err(ProtocolError("request counter".into()));
        }
        let n = id
            .parse::<u64>()
            .map_err(|_| ProtocolError("request counter".into()))?;
        if n > INTEGER_MAX || n != self.0 + 1 {
            return Err(ProtocolError("request order".into()));
        }
        self.0 = n;
        Ok(n)
    }
}

pub fn read_record<R: Read>(reader: &mut R, expected: u64) -> Result<(u8, Vec<u8>), ProtocolError> {
    let mut h = [0; 13];
    reader.read_exact(&mut h)?;
    let seq = u64::from_be_bytes(h[1..9].try_into().expect("fixed slice"));
    let len = u32::from_be_bytes(h[9..13].try_into().expect("fixed slice")) as usize;
    if seq != expected || !matches!(h[0], 1..=3) || len > TERM_CHUNK_MAX || (h[0] == 2 && len != 0)
    {
        return Err(ProtocolError("binary record".into()));
    }
    let mut data = vec![0; len];
    reader.read_exact(&mut data)?;
    if h[0] == 3 && std::str::from_utf8(&data).is_err() {
        return Err(ProtocolError("binary error must be UTF-8".into()));
    }
    Ok((h[0], data))
}
pub fn write_record<W: Write>(
    w: &mut W,
    kind: u8,
    sequence: u64,
    data: &[u8],
) -> Result<(), ProtocolError> {
    if data.len() > TERM_CHUNK_MAX || !matches!(kind, 1..=3) || (kind == 2 && !data.is_empty()) {
        return Err(ProtocolError("binary record".into()));
    }
    if kind == 3 && std::str::from_utf8(data).is_err() {
        return Err(ProtocolError("binary error must be UTF-8".into()));
    }
    w.write_all(&[kind])?;
    w.write_all(&sequence.to_be_bytes())?;
    w.write_all(&(data.len() as u32).to_be_bytes())?;
    w.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unicode_and_split() {
        let v = serde_json::json!({"text":"中文🦀"});
        let mut b = Vec::new();
        write_frame(&mut b, &v).unwrap();
        assert_eq!(
            read_frame(&mut io::BufReader::with_capacity(1, b.as_slice())).unwrap(),
            Some(v)
        );
    }
    #[test]
    fn rejects_bad_frames() {
        for b in [
            b"Content-Length: 2\n\n{}".as_slice(),
            b"Content-Length: 2\r\nX: x\r\n\r\n{}",
            b"Content-Length: 262145\r\n\r\n",
            b"Content-Length: 13\r\n\r\n{\"a\":1,\"a\":2}",
            b"Content-Length: 2\r\n\r\n[]",
        ] {
            assert!(read_frame(&mut io::Cursor::new(b)).is_err());
        }
    }
    #[test]
    fn bounds_depth_and_counter() {
        let s = format!("{}0{}", "[".repeat(34), "]".repeat(34));
        let b = format!("Content-Length: {}\r\n\r\n{}", s.len(), s);
        assert!(read_frame(&mut io::Cursor::new(b)).is_err());
        let mut c = RequestCounter::default();
        assert!(c.admit("2").is_err());
        assert_eq!(c.admit("1").unwrap(), 1);
        assert!(c.admit("1").is_err());
        assert_eq!(c.admit("2").unwrap(), 2);
    }
    #[test]
    fn binary_byte_preservation() {
        let mut b = Vec::new();
        write_record(&mut b, 1, 0, &[0, 255, 128, 13, 10]).unwrap();
        assert_eq!(
            read_record(&mut b.as_slice(), 0).unwrap().1,
            [0, 255, 128, 13, 10]
        );
        assert!(read_record(&mut b.as_slice(), 1).is_err());
    }
    #[test]
    fn binary_errors_require_utf8_without_restricting_data() {
        let mut malformed = Vec::new();
        write_record(&mut malformed, 1, 0, &[0xff]).unwrap();
        malformed[0] = 3;
        assert!(read_record(&mut malformed.as_slice(), 0).is_err());
        let mut output = Vec::new();
        assert!(write_record(&mut output, 3, 0, &[0xff]).is_err());
        assert!(output.is_empty());
        write_record(&mut output, 3, 0, b"DISPLAY_INCOMPLETE").unwrap();
        assert_eq!(
            read_record(&mut output.as_slice(), 0).unwrap().1,
            b"DISPLAY_INCOMPLETE"
        );
    }
}
