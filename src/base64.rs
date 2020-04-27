//! Base64 encoding and decoding

// I don't want to create a duplicate module here. But I can't get "crate" to work.
#[path = "math.rs"]
#[allow(dead_code)]
mod math;

use math::BYTE_BITS;

const B64_CHAR_BITS: usize = 6;

const B64_MAX: u8 = (1u8 << B64_CHAR_BITS) - 1;

const B64_BLOCK_BYTES: usize = 3;
const B64_BLOCK_CHARS: usize = 4;
const B64_BLOCK_BITS: usize = B64_BLOCK_BYTES * BYTE_BITS;

const MAX_B64_PAD_CHARS: usize = B64_BLOCK_BYTES - 1;
const B64_PAD_C: char = '=';
const B64_PAD_B: u8 = B64_PAD_C as u8;

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

fn base64_encode_block(block: &[u8], pad_count: usize) -> String {
    let mut s = String::with_capacity(B64_BLOCK_CHARS);
    assert!(block.len() == B64_BLOCK_BYTES);
    assert!(pad_count <= MAX_B64_PAD_CHARS);

    let b0 = block[0];
    let b1 = block[1];
    let b2 = block[2];

    let cb0 = (b0 & 0b11111100) >> 2;
    let cb1 = (b0 & 0b00000011) << 4 | (b1 & 0b11110000) >> 4;
    let cb2 = (b1 & 0b00001111) << 2 | (b2 & 0b11000000) >> 6;
    let cb3 = b2 & 0b00111111;

    s.push(base64_encode_char(cb0));
    s.push(base64_encode_char(cb1));
    // Special handling for padding
    if pad_count == 2 {
        assert!(cb2 == 0);
        s.push(B64_PAD_C);
    } else {
        s.push(base64_encode_char(cb2));
    }
    if pad_count >= 1 {
        assert!(cb3 == 0);
        s.push(B64_PAD_C);
    } else {
        s.push(base64_encode_char(cb3));
    }

    assert!(s.len() == B64_BLOCK_CHARS);
    s
}

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

fn base64_decode_block(block: &[u8]) -> Vec<u8> {
    let mut v = Vec::<u8>::with_capacity(B64_BLOCK_BYTES);
    // Require correct Base64 padding.
    // We might want to change this condition in future, to allow Base64 without padding.
    assert!(block.len() == B64_BLOCK_CHARS);

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

    // This isn't the best interface, but it's functional
    let cb0 = base64_decode_char(block[0] as char);
    let cb1 = base64_decode_char(block[1] as char);
    let cb2 = base64_decode_char(block[2] as char);
    let cb3 = base64_decode_char(block[3] as char);

    let b0 = (cb0 & 0b111111) << 2 | (cb1 & 0b110000) >> 4;
    let b1 = (cb1 & 0b001111) << 4 | (cb2 & 0b111100) >> 2;
    let b2 = (cb2 & 0b000011) << 6 | (cb3 & 0b111111);

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

pub fn base64_decode(s: &str) -> Vec<u8> {
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
    // Assume that the string is ASCII Base64, we'll check during conversion
    // This isn't the best interface, but it's functional
    let blocks = s.as_bytes().chunks(B64_BLOCK_CHARS);
    for block in blocks {
        // If we've found padding before, the Base64 is malformed
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
    #[should_panic(expected = "B64_BLOCK_CHARS")]
    fn invalid_base64_char_multibyte_encoded_utf8() {
        base64_decode("\u{00E9}");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
    fn invalid_base64_char_multibyte_encoded_utf8_twice() {
        base64_decode("\u{00E9}\u{00E9}");
    }

    #[test]
    #[should_panic(expected = "B64_BLOCK_CHARS")]
    fn invalid_base64_char_multibyte_decoded_utf8() {
        base64_decode("\u{2192}");
    }

    #[test]
    #[should_panic(expected = "Invalid Base64 character")]
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
