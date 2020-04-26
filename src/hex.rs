#[path = "math.rs"]
#[allow(dead_code)]
mod math;

use math::BYTE_BITS;

const HEX_CHAR_BITS: usize = 4;

const HEX_MAX: u8 = (1u8 << HEX_CHAR_BITS) - 1;

const HEX_BLOCK_BYTES: usize = 1;
const HEX_BLOCK_CHARS: usize = 2;
const HEX_BLOCK_BITS: usize = HEX_BLOCK_BYTES * BYTE_BITS;

fn hex_encode_char(char_bits: u8) -> char {
    // Implies char_bits.is_ascii()
    assert!(char_bits <= HEX_MAX);

    match char_bits {
        n @ 0..=9 => math::add_to_char('0', n),
        n @ 10..=HEX_MAX => math::add_to_char('a', n - 10),
        _ => panic!("unreachable"),
    }
}

fn hex_encode_block(block: u8) -> String {
    let mut s = String::with_capacity(HEX_BLOCK_CHARS);

    let cb0 = (block & 0b11110000) >> 4;
    let cb1 = block & 0b00001111;

    s.push(hex_encode_char(cb0));
    s.push(hex_encode_char(cb1));

    assert!(s.len() == HEX_BLOCK_CHARS);
    s
}

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

fn hex_decode_char(c: char) -> u8 {
    let b = match c {
        n @ '0'..='9' => math::char_diff(n, '0'),
        n @ 'a'..='z' => math::char_diff(n, 'a') + 10,
        // Also support uppercase hex
        n @ 'A'..='Z' => math::char_diff(n, 'A') + 10,
        _ => panic!("unreachable"),
    };

    assert!(b <= HEX_MAX);
    b
}

fn hex_decode_block(block: &[u8]) -> u8 {
    // Hex has no concept of padding, require whole blocks.
    // We might want to change this condition in future, to allow trailing hex nybbles.
    assert!(block.len() == HEX_BLOCK_CHARS);

    // This isn't the best interface, but it's functional
    let cb0 = hex_decode_char(block[0] as char);
    let cb1 = hex_decode_char(block[1] as char);

    (cb0 & 0b1111) << 4 | cb1 & 0b1111
}

pub fn hex_decode(s: &str) -> Vec<u8> {
    // Each 8 bit block turns 2 hex characters into 1 byte
    // This division must be exact.
    // We might want to change this condition in future, to allow trailing hex nybbles.
    let byte_count = math::exact_div(s.len() * HEX_CHAR_BITS, HEX_BLOCK_BITS);

    let mut v = Vec::<u8>::with_capacity(byte_count);
    // Assume that the string is ASCII hex, we'll check during conversion
    // This isn't the best interface, but it's functional
    let blocks = s.as_bytes().chunks(HEX_BLOCK_CHARS);
    for block in blocks {
        v.push(hex_decode_block(block));
    }

    assert!(v.len() == byte_count);
    v
}
