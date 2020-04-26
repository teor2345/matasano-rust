mod base64;
mod hex;
mod utf8;

fn main() {
    // UTF-8 encoding and decoding
    let empty = "";
    println!("Constant: '{}'", empty);

    let utf8_empty = utf8::utf8_encode(empty);
    println!("UTF-8 encoded: '{:?}'", utf8_empty);
    let rt_utf8_empty = utf8::utf8_decode(&utf8_empty);
    println!("UTF-8 decoded: '{}'", rt_utf8_empty);
    assert!(empty == rt_utf8_empty);

    let block = "\0@~";
    println!("Constant: '{}'", block);

    let utf8_block = utf8::utf8_encode(block);
    println!("UTF-8 encoded: '{:?}'", utf8_block);
    let rt_utf8_block = utf8::utf8_decode(&utf8_block);
    println!("UTF-8 decoded: '{}'", rt_utf8_block);
    assert!(block == rt_utf8_block);

    let hello = "Hello, world!";
    println!("Constant: '{}'", hello);

    let utf8_hello = utf8::utf8_encode(hello);
    println!("UTF-8 encoded: '{:?}'", utf8_hello);
    let rt_utf8_hello = utf8::utf8_decode(&utf8_hello);
    println!("UTF-8 decoded: '{}'", rt_utf8_hello);
    assert!(hello == rt_utf8_hello);

    // Base64 encoding and decoding
    let b64_utf8_empty = base64::base64_encode(&utf8_empty);
    println!("Base64 encoded UTF-8: '{}'", b64_utf8_empty);
    let rt_b64_utf8_empty = base64::base64_decode(&b64_utf8_empty);
    println!("Base64 decoded UTF-8: '{:?}'", rt_b64_utf8_empty);
    let rt_b64_empty = utf8::utf8_decode(&rt_b64_utf8_empty);
    println!("UTF-8 and Base64 decoded: '{}'", rt_b64_empty);
    assert!(utf8_empty == rt_b64_utf8_empty);
    assert!(empty == rt_b64_empty);

    let b64_utf8_block = base64::base64_encode(&utf8_block);
    println!("Base64 encoded UTF-8: '{}'", b64_utf8_block);
    let rt_b64_utf8_block = base64::base64_decode(&b64_utf8_block);
    println!("Base64 decoded UTF-8: '{:?}'", rt_b64_utf8_block);
    let rt_b64_block = utf8::utf8_decode(&rt_b64_utf8_block);
    println!("UTF-8 and Base64 decoded: '{}'", rt_b64_block);
    assert!(utf8_block == rt_b64_utf8_block);
    assert!(block == rt_b64_block);

    let b64_utf8_hello = base64::base64_encode(&utf8_hello);
    println!("Base64 encoded UTF-8: '{}'", b64_utf8_hello);
    let rt_b64_utf8_hello = base64::base64_decode(&b64_utf8_hello);
    println!("Base64 decoded UTF-8: '{:?}'", rt_b64_utf8_hello);
    let rt_b64_hello = utf8::utf8_decode(&rt_b64_utf8_hello);
    println!("UTF-8 and Base64 decoded: '{}'", rt_b64_hello);
    assert!(utf8_hello == rt_b64_utf8_hello);
    assert!(hello == rt_b64_hello);

    // Hex encoding and decoding
    let hex_utf8_empty = hex::hex_encode(&utf8_empty);
    println!("Hex encoded UTF-8: '{}'", hex_utf8_empty);
    let rt_hex_utf8_empty = hex::hex_decode(&hex_utf8_empty);
    println!("Hex decoded UTF-8: '{:?}'", rt_hex_utf8_empty);
    let rt_hex_empty = utf8::utf8_decode(&rt_hex_utf8_empty);
    println!("UTF-8 and Hex decoded: '{}'", rt_hex_empty);
    assert!(utf8_empty == rt_hex_utf8_empty);
    assert!(empty == rt_hex_empty);

    let hex_utf8_block = hex::hex_encode(&utf8_block);
    println!("Hex encoded UTF-8: '{}'", hex_utf8_block);
    let rt_hex_utf8_block = hex::hex_decode(&hex_utf8_block);
    println!("Hex decoded UTF-8: '{:?}'", rt_hex_utf8_block);
    let rt_hex_block = utf8::utf8_decode(&rt_hex_utf8_block);
    println!("UTF-8 and Hex decoded: '{}'", rt_hex_block);
    assert!(utf8_block == rt_hex_utf8_block);
    assert!(block == rt_hex_block);

    let hex_utf8_hello = hex::hex_encode(&utf8_hello);
    println!("Hex encoded UTF-8: '{}'", hex_utf8_hello);
    let rt_hex_utf8_hello = hex::hex_decode(&hex_utf8_hello);
    println!("Hex decoded UTF-8: '{:?}'", rt_hex_utf8_hello);
    let rt_hex_hello = utf8::utf8_decode(&rt_hex_utf8_hello);
    println!("UTF-8 and Hex decoded: '{}'", rt_hex_hello);
    assert!(utf8_hello == rt_hex_utf8_hello);
    assert!(hello == rt_hex_hello);
}
