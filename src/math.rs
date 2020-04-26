//! Integer and character mathematical utility functions

pub const BYTE_BITS: usize = 8;

pub fn ceil_div(n: usize, d: usize) -> usize {
    assert!(d > 0, "The divisor must not be zero");
    assert!(
        // n + d <= usize::MAX, but without overflow
        n <= usize::MAX - d,
        "The sum of the numerator {} and divisor {} must be less than or equal to usize::MAX ({})",
        n,
        d,
        usize::MAX
    );

    (n + d - 1) / d
}

pub fn exact_div(n: usize, d: usize) -> usize {
    assert!(d > 0, "The divisor must not be zero");
    assert!(
        n % d == 0,
        "Expected exact division, but {} / {} has remainder {}",
        n,
        d,
        n % d
    );

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
