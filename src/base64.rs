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
    // Implies char_bits.is_ascii()
    assert!(char_bits <= B64_MAX);

    match char_bits {
        n @ 0..=25 => math::add_to_char('A', n),
        n @ 26..=51 => math::add_to_char('a', n - 26),
        n @ 52..=61 => math::add_to_char('0', n - 52),
        62 => '+',
        63 => '/',
        _ => panic!("unreachable"),
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
        _ => panic!("unreachable"),
    };

    assert!(b <= B64_MAX);
    b
}

fn base64_decode_block(block: &[u8]) -> Vec<u8> {
    let mut v = Vec::<u8>::with_capacity(B64_BLOCK_BYTES);
    // Require correct Base64 padding.
    // We might want to change this condition in future, to allow Base64 without padding.
    assert!(block.len() == B64_BLOCK_CHARS);

    let pad_count = match block {
        [_, _, B64_PAD_B, B64_PAD_B] => 2,
        [_, _, _, B64_PAD_B] => 1,
        blk @ _ if blk.contains(&B64_PAD_B) => panic!("bad Base64 padding char"),
        _ => 0,
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
    }
    if pad_count == 0 {
        v.push(b2);
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
        assert!(!found_pad);

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
