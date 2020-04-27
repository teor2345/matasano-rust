//! Hex encoding and decoding

#![deny(missing_docs)]

// I don't want to create a duplicate module here. But I can't get "crate" to work.
#[path = "math.rs"]
#[allow(dead_code)]
mod math;

use math::BYTE_BITS;

/// The number of bits in a hex digit
const HEX_CHAR_BITS: usize = 4;

/// The maximum value of a hex digit
const HEX_MAX: u8 = (1u8 << HEX_CHAR_BITS) - 1;

/// The number of bytes in a hex conversion block
const HEX_BLOCK_BYTES: usize = 1;
/// The number of hex digits in a hex conversion block
const HEX_BLOCK_CHARS: usize = 2;
/// The number of bits in a hex conversion block
const HEX_BLOCK_BITS: usize = HEX_BLOCK_BYTES * BYTE_BITS;

/// Encode char_bits into a hex character.
/// Panics if char_bits is greater than HEX_MAX.
fn hex_encode_char(char_bits: u8) -> char {
    match char_bits {
        n @ 0..=9 => math::add_to_char('0', n),
        n @ 10..=HEX_MAX => math::add_to_char('a', n - 10),
        _ => unreachable!("Caller must ensure that char_bits <= HEX_MAX"),
    }
}

/// Encode a single-byte block, into a HEX_BLOCK_CHARS character hex string.
fn hex_encode_block(block: u8) -> String {
    let mut s = String::with_capacity(HEX_BLOCK_CHARS);

    let cb0 = (block & 0b11110000) >> 4;
    let cb1 = block & 0b00001111;

    s.push(hex_encode_char(cb0));
    s.push(hex_encode_char(cb1));

    assert!(s.len() == HEX_BLOCK_CHARS);
    s
}

/// Encode bytes into a hex string.
pub fn hex_encode(bytes: &[u8]) -> String {
    // Each 8 bit block turns 1 byte into 2 hex characters
    let hex_blocks = bytes.len();
    let char_count = hex_blocks * HEX_BLOCK_CHARS;

    let mut s = String::with_capacity(char_count);
    // Each byte is a block
    for block in bytes {
        s.push_str(&hex_encode_block(*block));
    }

    assert!(s.len() == char_count);
    s
}

/// Decode a hex character c into its corresponding HEX_CHAR_BITS bits.
/// Panics on non-hex characters, including (partial) multibyte characters.
fn hex_decode_char(c: char) -> u8 {
    let b = match c {
        n @ '0'..='9' => math::char_diff(n, '0'),
        n @ 'a'..='f' => math::char_diff(n, 'a') + 10,
        // Also support uppercase hex
        n @ 'A'..='F' => math::char_diff(n, 'A') + 10,
        _ => panic!("Invalid hex character"),
    };

    // Should be unreachable
    assert!(b <= HEX_MAX);
    b
}

/// Decode the HEX_BLOCK_CHARS hex characters in block, into a single byte.
/// block contains the byte values of the hex UTF-8 characters.
///
/// Panics if:
///  * block is not HEX_BLOCK_CHARS long, or
///  * block contains non-hex characters, including (partial) multibyte characters.
fn hex_decode_block(block: &[u8]) -> u8 {
    // Hex has no concept of padding, require whole blocks.
    // We might want to change this condition in future, to allow trailing hex nybbles.
    assert!(block.len() == HEX_BLOCK_CHARS);

    // The caller should ensure that these are ASCII
    assert!((block[0] as char).is_ascii());
    assert!((block[1] as char).is_ascii());

    let cb0 = hex_decode_char(block[0] as char);
    let cb1 = hex_decode_char(block[1] as char);

    (cb0 & 0b1111) << 4 | cb1 & 0b1111
}

/// Decode a hex string s into bytes.
///
/// Panics if:
///  * s is not a multiple of HEX_BLOCK_CHARS long,
///  * s contains non-hex characters, including multibyte characters.
pub fn hex_decode(s: &str) -> Vec<u8> {
    // Hex strings must be ASCII
    assert!(s.is_ascii(), "Invalid hex string");

    // Each 8 bit block turns 2 hex characters into 1 byte
    // This division must be exact.
    // We might want to change this condition in future, to allow trailing hex nybbles.
    let byte_count = math::exact_div(s.len() * HEX_CHAR_BITS, HEX_BLOCK_BITS);

    let mut v = Vec::<u8>::with_capacity(byte_count);
    // Since the string is ASCII, we can safely iterate over its bytes.
    let blocks = s.as_bytes().chunks(HEX_BLOCK_CHARS);
    for block in blocks {
        v.push(hex_decode_block(block));
    }

    assert!(v.len() == byte_count);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        assert_eq!(hex_encode(&[0]), "00");
        assert_eq!(hex_encode(&[9]), "09");
        assert_eq!(hex_encode(&[10]), "0a");
        assert_eq!(hex_encode(&[15]), "0f");
        assert_eq!(hex_encode(&[16]), "10");
        assert_eq!(hex_encode(&[26]), "1a");
        assert_eq!(hex_encode(&[160]), "a0");
        assert_eq!(hex_encode(&[255]), "ff");

        assert_eq!(hex_decode("00"), [0]);
        assert_eq!(hex_decode("09"), [9]);
        assert_eq!(hex_decode("0a"), [10]);
        assert_eq!(hex_decode("0f"), [15]);
        assert_eq!(hex_decode("10"), [16]);
        assert_eq!(hex_decode("90"), [144]);
        assert_eq!(hex_decode("a0"), [160]);
        assert_eq!(hex_decode("f0"), [240]);
        assert_eq!(hex_decode("ff"), [255]);
    }

    #[test]
    fn decode_uppercase() {
        assert_eq!(hex_decode("0A"), [10]);
        assert_eq!(hex_decode("0F"), [15]);
        assert_eq!(hex_decode("A0"), [160]);
        assert_eq!(hex_decode("F0"), [240]);
        assert_eq!(hex_decode("FF"), [255]);
    }

    #[test]
    fn empty() {
        assert_eq!(hex_encode(&[]), "");
        assert_eq!(hex_decode(""), []);
    }

    #[test]
    fn multi_block_string() {
        assert_eq!(hex_decode("ffff"), [255, 255]);
        assert_eq!(hex_encode(&[255, 255]), "ffff");

        assert_eq!(hex_decode("1002"), [16, 2]);
        assert_eq!(hex_encode(&[16, 2]), "1002");
    }

    #[test]
    fn round_trip() {
        let test_vector = "eeaf0023";
        assert_eq!(hex_encode(&hex_decode(test_vector)), test_vector);
    }

    #[test]
    #[should_panic(expected = "Invalid hex character")]
    fn invalid_hex_char_before_0() {
        hex_decode("//");
    }

    #[test]
    #[should_panic(expected = "Invalid hex character")]
    fn invalid_hex_char_after_9() {
        hex_decode("::");
    }

    #[test]
    #[should_panic(expected = "Invalid hex character")]
    #[allow(non_snake_case)]
    fn invalid_hex_char_before_A() {
        hex_decode("@@");
    }

    #[test]
    #[should_panic(expected = "Invalid hex character")]
    #[allow(non_snake_case)]
    fn invalid_hex_char_after_F() {
        hex_decode("GG");
    }

    #[test]
    #[should_panic(expected = "Invalid hex character")]
    fn invalid_hex_char_before_a() {
        hex_decode("``");
    }

    #[test]
    #[should_panic(expected = "Invalid hex character")]
    fn invalid_hex_char_after_f() {
        hex_decode("gg");
    }

    #[test]
    #[should_panic(expected = "Invalid hex string")]
    fn invalid_hex_char_multibyte_encoded_utf8() {
        hex_decode("\u{00E9}");
    }

    #[test]
    #[should_panic(expected = "Invalid hex string")]
    fn invalid_hex_char_multibyte_decoded_utf8() {
        hex_decode("\u{2192}");
    }

    #[test]
    #[should_panic(expected = "Expected exact division")]
    fn invalid_hex_truncated() {
        hex_decode("f");
    }

    #[test]
    #[should_panic(expected = "Expected exact division")]
    fn invalid_hex_truncated_multiblock() {
        hex_decode("aaa");
    }

    // Encoding out-of-range integers won't compile
}
