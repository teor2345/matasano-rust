//! UTF-8 encoding and decoding

pub fn utf8_encode(s: &str) -> Vec<u8> {
    s.bytes().collect()
}

pub fn utf8_decode(utf8_bytes: &[u8]) -> String {
    let r = String::from_utf8(utf8_bytes.to_vec());
    r.expect("utf8_bytes must be valid UTF-8")
}
