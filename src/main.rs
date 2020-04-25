fn utf8_encode(s: &str) -> Vec<u8> {
    s.bytes().collect()
}

fn utf8_decode(utf8_bytes: &[u8]) -> String {
    let r = String::from_utf8(utf8_bytes.to_vec());
    r.expect("utf8_bytes must be valid UTF-8")
}

fn main() {
    let hello = "Hello, world!";
    println!("Constant: '{}'", hello);

    let utf8_hello = utf8_encode(hello);
    println!("UTF-8 encoded: '{:?}'", utf8_hello);
    let rt_utf8_hello = utf8_decode(&utf8_hello);
    println!("UTF-8 decoded: '{}'", rt_utf8_hello);
    assert!(hello == rt_utf8_hello);
}
