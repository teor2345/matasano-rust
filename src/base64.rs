//! Base64 encoding and decoding

#![deny(missing_docs)]

// I don't want to create a duplicate module here. But I can't get "crate" to work.
#[path = "math.rs"]
#[allow(dead_code)]
mod math;

use math::BYTE_BITS;

/// The number of bits in a Base64 digit
const B64_CHAR_BITS: usize = 6;

/// The maximum value of a Base64 digit
const B64_MAX: u8 = (1u8 << B64_CHAR_BITS) - 1;

/// The number of bytes in a Base64 conversion block
const B64_BLOCK_BYTES: usize = 3;
/// The number of Base64 digits in a Base64 conversion block
const B64_BLOCK_CHARS: usize = 4;
/// The number of bits in a Base64 conversion block
const B64_BLOCK_BITS: usize = B64_BLOCK_BYTES * BYTE_BITS;

/// The maximum number of Base64 padding characters at the end of the final block
const MAX_B64_PAD_CHARS: usize = B64_BLOCK_BYTES - 1;
/// The Base64 padding character
const B64_PAD_C: char = '=';
/// The Base64 padding character, as a byte
const B64_PAD_B: u8 = B64_PAD_C as u8;

/// Encode char_bits into a Base64 character.
/// Panics if char_bits is greater than B64_MAX.
fn base64_encode_char(char_bits: u8) -> char {
    match char_bits {
        n @ 0..=25 => math::add_to_char('A', n),
        n @ 26..=51 => math::add_to_char('a', n - 26),
        n @ 52..=61 => math::add_to_char('0', n - 52),
        62 => '+',
        63 => '/',
        _ => unreachable!("Caller ensures that char_bits <= B64_MAX"),
    }
}

/// Encode the B64_BLOCK_BYTES bytes in block, into a B64_BLOCK_CHARS character Base64 string.
///
/// If the block is less than B64_BLOCK_BYTES long, the caller must zero-fill it.
/// pad_count is the number of bytes that were zero-filled. The returned string is padded with that
/// many B64_PAD_C padding characters.
///
/// Panics if:
///  * block is not B64_BLOCK_BYTES long. or
///  * pad_count is greater than MAX_B64_PAD_CHARS
fn base64_encode_block(block: &[u8], pad_count: usize) -> String {
    assert!(block.len() == B64_BLOCK_BYTES);
    assert!(pad_count <= MAX_B64_PAD_CHARS);

    let mut s = String::with_capacity(B64_BLOCK_CHARS);
    let mut c = Vec::<u8>::with_capacity(B64_BLOCK_CHARS);

    c.push((block[0] & 0b11111100) >> 2);
    c.push((block[0] & 0b00000011) << 4 | (block[1] & 0b11110000) >> 4);
    c.push((block[1] & 0b00001111) << 2 | (block[2] & 0b11000000) >> 6);
    c.push(block[2] & 0b00111111);

    s.push(base64_encode_char(c[0]));
    s.push(base64_encode_char(c[1]));
    // Special handling for padding
    if pad_count == 2 {
        assert!(c[2] == 0);
        s.push(B64_PAD_C);
    } else {
        s.push(base64_encode_char(c[2]));
    }
    if pad_count >= 1 {
        assert!(c[3] == 0);
        s.push(B64_PAD_C);
    } else {
        s.push(base64_encode_char(c[3]));
    }

    assert!(s.len() == B64_BLOCK_CHARS);
    s
}

/// Encode bytes into a Base64 string.
///
/// If bytes is not a multiple of B64_BLOCK_BYTES long, the returned string is padded with a
/// B64_PAD_C padding character for each missing byte.
pub fn base64_encode(bytes: &[u8]) -> String {
    // Each 24 bit block turns 3 bytes into 4 base64 characters
    // Round up the number of blocks
    let b64_blocks = math::ceil_div(bytes.len() * BYTE_BITS, B64_BLOCK_BITS);
    let char_count = b64_blocks * B64_BLOCK_CHARS;

    let mut s = String::with_capacity(char_count);
    let blocks = bytes.chunks(B64_BLOCK_BYTES);
    for block in blocks {
        if block.len() == B64_BLOCK_BYTES {
            s.push_str(&base64_encode_block(&block, 0));
        } else if block.len() > 0 {
            let pad_count: usize = B64_BLOCK_BYTES - block.len();
            let mut v = block.to_vec();
            v.resize(B64_BLOCK_BYTES, 0);
            s.push_str(&base64_encode_block(&v, pad_count));
        }
    }

    assert!(s.len() == char_count);
    s
}

/// Decode a Base64 character c into its corresponding B64_CHAR_BITS bits.
/// Panics on non-Base64 characters, including (partial) multibyte characters.
fn base64_decode_char(c: char) -> u8 {
    let b = match c {
        n @ 'A'..='Z' => math::char_diff(n, 'A'),
        n @ 'a'..='z' => math::char_diff(n, 'a') + 26,
        n @ '0'..='9' => math::char_diff(n, '0') + 52,
        '+' => 62,
        '/' => 63,
        // Special case for padding
        '=' => 0,
        _ => panic!("Invalid Base64 character"),
    };

    // Should be unreachable
    assert!(b <= B64_MAX);
    b
}

/// Decode the B64_BLOCK_CHARS Base64 characters in block, into B64_BLOCK_BYTES bytes.
/// block contains the byte values of the Base64 UTF-8 characters.
///
/// Panics if:
///  * block is not B64_BLOCK_CHARS long,
///  * block does not have correct Base64 padding,
///  * block contains non-Base64 characters, including (partial) multibyte characters, or
///  * block is padded correctly, but the Base64 characters in block leave trailing bits in the padding
///    bytes.
fn base64_decode_block(block: &[u8]) -> Vec<u8> {
    // Require correct Base64 padding.
    // We might want to change this condition in future, to allow Base64 without padding.
    assert!(block.len() == B64_BLOCK_CHARS);

    // The caller should ensure that these are ASCII
    assert!(block.iter().all(|b| (*b as char).is_ascii()));

    // There are only two valid paddings:
    // ???=
    // ??==
    // where ? is a non-padding character.
    let pad_count = match block {
        // The order of these patterns is significant
        // Never allowed in the first two characters
        [B64_PAD_B, _, _, _] => panic!("Invalid Base64 padding position in {:?}", block),
        [_, B64_PAD_B, _, _] => panic!("Invalid Base64 padding position in {:?}", block),
        // Only allowed in the second-last character, if the last character is also padding
        [_, _, B64_PAD_B, B64_PAD_B] => 2,
        [_, _, B64_PAD_B, _] => panic!("Invalid Base64 padding position in {:?}", block),
        // Allowed in the last character, if previous conditions don't match
        [_, _, _, B64_PAD_B] => 1,
        // There shouldn't be any padding at this point
        _ if block.contains(&B64_PAD_B) => {
            unreachable!("Previous cases should exhaustively cover all padding")
        }
        // No padding, if previous conditions don't match
        [_, _, _, _] => 0,
        // The previous conditions should be exhaustive
        _ => unreachable!("block should be a 4-item slice"),
    };

    let c: Vec<u8> = block
        .iter()
        .map(|b| base64_decode_char(*b as char))
        .collect();

    let b0 = (c[0] & 0b111111) << 2 | (c[1] & 0b110000) >> 4;
    let b1 = (c[1] & 0b001111) << 4 | (c[2] & 0b111100) >> 2;
    let b2 = (c[2] & 0b000011) << 6 | (c[3] & 0b111111);

    let mut v = Vec::<u8>::with_capacity(B64_BLOCK_BYTES);

    v.push(b0);
    if pad_count < 2 {
        v.push(b1);
    } else {
        assert!(
            b1 == 0,
            "Trailing Base64 bits ignored in last character due to padding"
        );
    }
    if pad_count == 0 {
        v.push(b2);
    } else {
        assert!(
            b2 == 0,
            "Trailing Base64 bits ignored in last character due to padding"
        );
    }

    assert!(v.len() > 0);
    assert!(v.len() <= B64_BLOCK_BYTES);
    v
}

/// Decode a Base64 string s into bytes.
///
/// Panics if:
///  * s is not a multiple of B64_BLOCK_CHARS long,
///  * the final block in s does not have correct Base64 padding,
///  * non-terminal blocks in s have Base64 padding,
///  * s contains non-Base64 characters, including multibyte characters, or
///  * s is padded correctly, but the Base64 characters in block leave trailing bits in the padding
///    bytes.
pub fn base64_decode(s: &str) -> Vec<u8> {
    // Base64 strings must be ASCII
    assert!(s.is_ascii(), "Invalid Base64 string");

    // Each 24 bit block turns 4 base64 characters into 3 bytes
    // Round up the number of blocks
    let b64_blocks = math::ceil_div(s.len() * B64_CHAR_BITS, B64_BLOCK_BITS);
    let max_byte_count = b64_blocks * B64_BLOCK_BYTES;
    let min_byte_count = match b64_blocks {
        0 => 0,
        _ => max_byte_count - MAX_B64_PAD_CHARS,
    };

    let mut found_pad = false;
    let mut v = Vec::<u8>::with_capacity(max_byte_count);
    // Since the string is ASCII, we can safely iterate over its bytes.
    let blocks = s.as_bytes().chunks(B64_BLOCK_CHARS);
    for block in blocks {
        // If we've found padding in a previous block, the Base64 is malformed
        assert!(!found_pad, "Invalid Base64 padding in mid-stream block");

        let mut r = base64_decode_block(&block);
        assert!(r.len() <= B64_BLOCK_BYTES);
        assert!(r.len() > 0);
        found_pad = r.len() < B64_BLOCK_BYTES;
        v.append(&mut r);
    }

    assert!(v.len() <= max_byte_count);
    assert!(v.len() >= min_byte_count);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        assert_eq!(base64_decode("AAAA"), [0, 0, 0]);
        assert_eq!(base64_decode("ZAAA"), [25 << 2, 0, 0]);
        assert_eq!(base64_decode("aAAA"), [26 << 2, 0, 0]);
        assert_eq!(base64_decode("zAAA"), [51 << 2, 0, 0]);
        assert_eq!(base64_decode("0AAA"), [52 << 2, 0, 0]);
        assert_eq!(base64_decode("9AAA"), [61 << 2, 0, 0]);
        assert_eq!(base64_decode("+AAA"), [62 << 2, 0, 0]);
        assert_eq!(base64_decode("/AAA"), [63 << 2, 0, 0]);

        assert_eq!(base64_encode(&[0, 0, 0]), "AAAA");
        assert_eq!(base64_encode(&[0, 0, 25]), "AAAZ");
        assert_eq!(base64_encode(&[0, 0, 26]), "AAAa");
        assert_eq!(base64_encode(&[0, 0, 51]), "AAAz");
        assert_eq!(base64_encode(&[0, 0, 52]), "AAA0");
        assert_eq!(base64_encode(&[0, 0, 61]), "AAA9");
        assert_eq!(base64_encode(&[0, 0, 62]), "AAA+");
        assert_eq!(base64_encode(&[0, 0, 63]), "AAA/");
    }

    #[test]
    fn empty() {
        assert_eq!(base64_encode(&[]), "");
        assert_eq!(base64_decode(""), []);
    }

    #[test]
    fn multi_block_string() {
        assert_eq!(base64_decode("////AAAA"), [255, 255, 255, 0, 0, 0]);
        assert_eq!(base64_encode(&[255, 255, 255, 0, 0, 0]), "////AAAA");

        assert_eq!(base64_decode("BAAAAAAC"), [1 << 2, 0, 0, 0, 0, 2]);
        assert_eq!(base64_encode(&[1 << 2, 0, 0, 0, 0, 2]), "BAAAAAAC");
    }

    #[test]
    fn padding() {
        assert_eq!(base64_encode(&[0]), "AA==");
        assert_eq!(base64_encode(&[0, 0]), "AAA=");

        assert_eq!(base64_decode("gA=="), [32 << 2]);
        assert_eq!(base64_decode("gg=="), [(32 << 2) + (32 >> 4)]);

        assert_eq!(base64_decode("AAg="), [0, 32 >> 2]);
        assert_eq!(base64_decode("Agg="), [32 >> 4, 32 >> 2]);
    }

    #[test]
    fn round_trip() {
        let test_vector = "AZaz09+/";
        assert_eq!(base64_encode(&base64_decode(test_vector)), test_vector);
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    fn invalid_base64_char_before_plus() {
        base64_decode("****");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    fn invalid_base64_char_after_plus() {
        base64_decode(",,,,");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    fn invalid_base64_char_before_slash() {
        base64_decode("....");
    }

    // In ASCII, slash is before 0

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    fn invalid_base64_char_after_9() {
        base64_decode("::::");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    #[allow(non_snake_case)]
    fn invalid_base64_char_before_A() {
        base64_decode("@@@@");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    #[allow(non_snake_case)]
    fn invalid_base64_char_after_Z() {
        base64_decode("[[[[");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    fn invalid_base64_char_before_a() {
        base64_decode("````");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    fn invalid_base64_char_after_z() {
        base64_decode("{{{{");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 string")]
    fn invalid_base64_char_multibyte_encoded_utf8() {
        base64_decode("\u{00E9}");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 string")]
    fn invalid_base64_char_multibyte_encoded_utf8_twice() {
        base64_decode("\u{00E9}\u{00E9}");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 string")]
    fn invalid_base64_char_multibyte_decoded_utf8() {
        base64_decode("\u{2192}");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 string")]
    fn invalid_base64_char_multibyte_decoded_utf8_twice() {
        base64_decode("\u{2192}\u{2192}");
    }

    #[test]
    #[should_panic(expected = "B64_BLOCK_CHARS")]
    fn invalid_base64_truncated() {
        base64_decode("A");
    }

    #[test]
    #[should_panic(expected = "B64_BLOCK_CHARS")]
    fn invalid_base64_truncated_multiblock() {
        base64_decode("AAAAAA");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 padding position")]
    fn invalid_base64_pad_first_in_block() {
        base64_decode("====");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 padding position")]
    fn invalid_base64_pad_second_in_block() {
        base64_decode("A===");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 padding position")]
    fn invalid_base64_pad_mid_block() {
        base64_decode("AA=A");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 padding in mid-stream block")]
    fn invalid_base64_pad_mid_stream() {
        base64_decode("AAA=AAAA");
    }

    #[test]
    #[should_panic(expected = "B64_BLOCK_CHARS")]
    fn invalid_base64_pad_trailing_block() {
        base64_decode("AAAA=");
    }

    #[test]
    #[should_panic(expected = "Trailing Base64 bits ignored")]
    fn invalid_base64_pad_trailing_bits_1_byte() {
        // This decodes to 0b100000 0b000001, but the output is only one byte, so the final 4 bits must be zero
        base64_decode("gB==");
    }

    #[test]
    #[should_panic(expected = "Trailing Base64 bits ignored")]
    fn invalid_base64_pad_trailing_bits_2_bytes() {
        // This decodes to 0b100000 0b000000 0b000010, but the output is only two bytes, so the final two bits must be zero
        base64_decode("gAC=");
    }

    // Encoding out-of-range integers won't compile
}
