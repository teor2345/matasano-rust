fn utf8_encode(s: &str) -> Vec<u8> {
    s.bytes().collect()
}

fn utf8_decode(utf8_bytes: &[u8]) -> String {
    let r = String::from_utf8(utf8_bytes.to_vec());
    r.expect("utf8_bytes must be valid UTF-8")
}

const BYTE_BITS: usize = 8;
const B64_CHAR_BITS: usize = 6;

const B64_MAX: u8 = (1u8 << B64_CHAR_BITS) - 1;

const B64_BLOCK_BYTES: usize = 3;
const B64_BLOCK_CHARS: usize = 4;
const B64_BLOCK_BITS: usize = B64_BLOCK_BYTES * BYTE_BITS;

const MAX_B64_PAD_CHARS: usize = B64_BLOCK_BYTES - 1;
const B64_PAD_C: char = '=';

fn ceil_div(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}

fn add_to_char(c: char, n: u8) -> char {
    assert!(c.is_ascii());
    let r = ((c as u8) + n) as char;

    assert!(r.is_ascii());
    r
}

fn base64_encode_char(char_bits: u8) -> char {
    // Implies char_bits.is_ascii()
    assert!(char_bits <= B64_MAX);

    match char_bits {
        n @ 0..=25 => add_to_char('A', n),
        n @ 26..=51 => add_to_char('a', n - 26),
        n @ 52..=61 => add_to_char('0', n - 52),
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

fn base64_encode(bytes: &[u8]) -> String {
    // Each 24 bit block turns 3 bytes into 4 base64 characters
    // Round up the number of blocks
    let b64_blocks = ceil_div(bytes.len() * BYTE_BITS, B64_BLOCK_BITS);
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

fn main() {
    // UTF-8 encoding and decoding
    let empty = "";
    println!("Constant: '{}'", empty);

    let utf8_empty = utf8_encode(empty);
    println!("UTF-8 encoded: '{:?}'", utf8_empty);
    let rt_utf8_empty = utf8_decode(&utf8_empty);
    println!("UTF-8 decoded: '{}'", rt_utf8_empty);
    assert!(empty == rt_utf8_empty);

    let block = "\0@~";
    println!("Constant: '{}'", block);

    let utf8_block = utf8_encode(block);
    println!("UTF-8 encoded: '{:?}'", utf8_block);
    let rt_utf8_block = utf8_decode(&utf8_block);
    println!("UTF-8 decoded: '{}'", rt_utf8_block);
    assert!(block == rt_utf8_block);

    let hello = "Hello, world!";
    println!("Constant: '{}'", hello);

    let utf8_hello = utf8_encode(hello);
    println!("UTF-8 encoded: '{:?}'", utf8_hello);
    let rt_utf8_hello = utf8_decode(&utf8_hello);
    println!("UTF-8 decoded: '{}'", rt_utf8_hello);
    assert!(hello == rt_utf8_hello);

    // Base64 encoding
    let b64_utf8_empty = base64_encode(&utf8_empty);
    println!("Base64 encoded UTF-8: '{}'", b64_utf8_empty);

    let b64_utf8_block = base64_encode(&utf8_block);
    println!("Base64 encoded UTF-8: '{}'", b64_utf8_block);

    let b64_utf8_hello = base64_encode(&utf8_hello);
    println!("Base64 encoded UTF-8: '{}'", b64_utf8_hello);
}
