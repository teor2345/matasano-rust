pub const BYTE_BITS: usize = 8;

pub fn ceil_div(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}

pub fn exact_div(n: usize, d: usize) -> usize {
    assert!(n % d == 0);
    n / d
}

pub fn add_to_char(c: char, n: u8) -> char {
    assert!(c.is_ascii());
    let r = ((c as u8) + n) as char;

    assert!(r.is_ascii());
    r
}

pub fn char_diff(c: char, base_char: char) -> u8 {
    assert!(c.is_ascii());
    assert!(base_char.is_ascii());

    (c as u8) - (base_char as u8)
}
