//! UTF-8 encoding and decoding

pub fn utf8_encode(s: &str) -> Vec<u8> {
    s.bytes().collect()
}

pub fn utf8_decode(utf8_bytes: &[u8]) -> String {
    let r = String::from_utf8(utf8_bytes.to_vec());
    r.expect("utf8_bytes must be valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii() {
        assert_eq!(utf8_encode(" "), [32]);
        assert_eq!(utf8_encode("@"), [64]);
        assert_eq!(utf8_encode("a"), [97]);
        assert_eq!(utf8_encode("~"), [126]);
    }

    #[test]
    fn empty() {
        assert_eq!(utf8_encode(""), []);
        assert_eq!(utf8_decode(&[]), "");
    }

    #[test]
    fn nul() {
        // Unlike C, Rust supports nul bytes / chars in its UTF-8 strings
        assert_eq!(utf8_encode("\0"), [0]);
        assert_eq!(utf8_decode(&[0]), "\0");
    }

    #[test]
    fn multi_char_string() {
        assert_eq!(utf8_encode("Test"), [84, 101, 115, 116]);
        assert_eq!(utf8_decode(&[84, 101, 115, 116]), "Test");
    }

    #[test]
    fn round_trip() {
        let hello = "Hello World!";
        assert_eq!(utf8_decode(&utf8_encode(hello)), hello);
    }

    #[test]
    fn multibyte_char() {
        assert_eq!(utf8_encode("é"), [0b11000011, 0b10101001]);
        assert_eq!(utf8_decode(&[0b11000011, 0b10101001]), "é");

        assert_eq!(utf8_encode("Caféteria"),
                   [67, 97, 102, 0b11000011, 0b10101001, 116, 101, 114, 105, 97]);
        assert_eq!(utf8_decode(&[67, 97, 102, 0b11000011, 0b10101001, 116, 101, 114, 105, 97]),
                   "Caféteria");
    }

    #[test]
    #[should_panic(expected = "must be valid UTF-8")]
    fn invalid_utf8_overlong_ascii() {
        utf8_decode(&[0xC0]);
    }

    #[test]
    #[should_panic(expected = "must be valid UTF-8")]
    fn invalid_utf8_undefined() {
        utf8_decode(&[0xFF]);
    }

    #[test]
    #[should_panic(expected = "must be valid UTF-8")]
    fn invalid_utf8_truncated() {
        utf8_decode(&[0xC3]);
    }

    #[test]
    #[should_panic(expected = "must be valid UTF-8")]
    fn invalid_utf8_bad_continuation() {
        utf8_decode(&[0xC3, 0xC3]);
    }

    // Decoding out-of-range integers won't compile
}
