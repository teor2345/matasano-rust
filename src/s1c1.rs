//! Set 1, Challenge 1, Matasano Cryptopals Challenges

// I don't want to create duplicate modules here. But I can't get "crate" to work.
#[path = "base64.rs"]
#[allow(dead_code)]
mod base64;
#[path = "hex.rs"]
#[allow(dead_code)]
mod hex;

// Test vectors for 1.1
const HEX_TEST: &'static str =
    "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const B64_EXPECTED_TEST_OUTPUT: &'static str =
    "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

pub fn check() {
    println!("Hex test: '{}'", HEX_TEST);
    let bytes_test = hex::hex_decode(&HEX_TEST);
    println!("Hex decoded test: '{:?}'", bytes_test);
    let b64_test = base64::base64_encode(&bytes_test);
    println!("Base64 encoded test: '{}'", b64_test);
    println!("Base64 expected output: '{}'", B64_EXPECTED_TEST_OUTPUT);
    assert!(b64_test == B64_EXPECTED_TEST_OUTPUT);
}
