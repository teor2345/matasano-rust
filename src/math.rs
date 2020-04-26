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
    assert!(
        c >= base_char,
        "c must be greater than or equal to base_char, when encoding using ASCII"
    );

    (c as u8) - (base_char as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_ranges() {
        assert_eq!(add_to_char('\0', 0), '\0');
        assert_eq!(add_to_char('\0', 32), ' ');
        assert_eq!(add_to_char('\0', 126), '~');

        assert_eq!(add_to_char('}', 0), '}');
        assert_eq!(add_to_char('}', 1), '~');
        assert_eq!(add_to_char('~', 0), '~');

        assert_eq!(char_diff('\0', '\0'), 0);
        assert_eq!(char_diff('~', '~'), 0);

        assert_eq!(char_diff(' ', '\0'), 32);
        assert_eq!(char_diff('~', '\0'), 126);

        assert_eq!(char_diff('~', '}'), 1);
    }

    #[test]
    fn div_ranges() {
        // 0, 1, MAX
        assert_eq!(ceil_div(0, 1), 0);
        assert_eq!(exact_div(0, 1), 0);

        assert_eq!(ceil_div(0, usize::MAX), 0);
        assert_eq!(exact_div(0, usize::MAX), 0);

        assert_eq!(exact_div(usize::MAX, 1), usize::MAX);

        assert_eq!(exact_div(usize::MAX, usize::MAX), 1);

        // 1, 2
        assert_eq!(ceil_div(1, 1), 1);
        assert_eq!(exact_div(1, 1), 1);

        assert_eq!(ceil_div(2, 2), 1);
        assert_eq!(exact_div(2, 2), 1);

        assert_eq!(ceil_div(1, 2), 1);

        assert_eq!(ceil_div(2, 1), 2);
        assert_eq!(exact_div(2, 1), 2);

        assert_eq!(ceil_div(6, 3), 2);
        assert_eq!(exact_div(6, 3), 2);

        assert_eq!(ceil_div(5, 3), 2);

        // (MAX - 1), 2
        assert_eq!(ceil_div(0, usize::MAX - 1), 0);
        assert_eq!(exact_div(0, usize::MAX - 1), 0);

        assert_eq!(ceil_div(1, usize::MAX - 1), 1);

        assert_eq!(ceil_div(usize::MAX - 1, 1), usize::MAX - 1);
        assert_eq!(exact_div(usize::MAX - 1, 1), usize::MAX - 1);

        assert_eq!(exact_div(usize::MAX - 1, usize::MAX - 1), 1);

        let floor_half_max = (usize::MAX - 1) / 2;
        let ceil_half_max = (usize::MAX - 1) / 2 + 1;

        assert_eq!(exact_div(usize::MAX - 1, 2), floor_half_max);

        assert_eq!(ceil_div(usize::MAX - 2, 2), floor_half_max);

        assert_eq!(ceil_div(floor_half_max, floor_half_max), 1);
        assert_eq!(exact_div(floor_half_max, floor_half_max), 1);

        assert_eq!(exact_div(ceil_half_max, ceil_half_max), 1);
    }

    #[test]
    fn round_trip_char() {
        let c = 'A';
        let n = 7;
        assert_eq!(char_diff(add_to_char(c, n), c), n);
    }

    #[test]
    #[should_panic(expected = "divisor must not be zero")]
    fn invalid_div_exact_one_div_zero() {
        exact_div(1, 0);
    }

    #[test]
    #[should_panic(expected = "divisor must not be zero")]
    fn invalid_div_exact_zero_div_zero() {
        exact_div(0, 0);
    }

    #[test]
    #[should_panic(expected = "divisor must not be zero")]
    fn invalid_div_ceil_one_div_zero() {
        ceil_div(1, 0);
    }

    #[test]
    #[should_panic(expected = "divisor must not be zero")]
    fn invalid_div_ceil_zero_div_zero() {
        ceil_div(0, 0);
    }

    #[test]
    #[should_panic(expected = "must be less than or equal to usize::MAX")]
    fn invalid_div_ceil_overflow_1_max() {
        ceil_div(1, usize::MAX);
    }

    #[test]
    #[should_panic(expected = "must be less than or equal to usize::MAX")]
    fn invalid_div_ceil_overflow_max_1() {
        ceil_div(usize::MAX, 1);
    }

    #[test]
    #[should_panic(expected = "must be less than or equal to usize::MAX")]
    fn invalid_div_ceil_overflow_max_max() {
        ceil_div(usize::MAX, usize::MAX);
    }

    #[test]
    #[should_panic(expected = "must be less than or equal to usize::MAX")]
    fn invalid_div_ceil_overflow_ceil_max_minus_one() {
        ceil_div(usize::MAX - 1, usize::MAX - 1);
    }

    #[test]
    #[should_panic(expected = "must be less than or equal to usize::MAX")]
    fn invalid_div_ceil_overflow_ceil_max_div_two() {
        ceil_div(usize::MAX, 2);
    }

    #[test]
    #[should_panic(expected = "must be less than or equal to usize::MAX")]
    fn invalid_div_ceil_overflow_ceil_max_minus_one_div_two() {
        ceil_div(usize::MAX - 1, 2);
    }

    #[test]
    #[should_panic(expected = "must be less than or equal to usize::MAX")]
    fn invalid_div_ceil_overflow_ceil_half_max() {
        let ceil_half_max = (usize::MAX - 1) / 2 + 1;
        ceil_div(ceil_half_max, ceil_half_max);
    }

    #[test]
    #[should_panic(expected = "Expected exact division")]
    fn invalid_div_inexact_max() {
        exact_div(1, usize::MAX);
    }

    #[test]
    #[should_panic(expected = "Expected exact division")]
    fn invalid_div_inexact_min() {
        exact_div(1, 2);
    }

    #[test]
    #[should_panic(expected = "Expected exact division")]
    fn invalid_div_inexact_small() {
        exact_div(5, 3);
    }

    #[test]
    #[should_panic(expected = "Expected exact division")]
    fn invalid_div_inexact_large() {
        exact_div(1, usize::MAX - 1);
    }

    #[test]
    #[should_panic(expected = "Expected exact division")]
    fn invalid_div_inexact_large_odd() {
        exact_div(usize::MAX, 2);
    }

    #[test]
    #[should_panic(expected = "is_ascii")]
    fn invalid_add_to_char_input_not_ascii() {
        add_to_char('\u{00E9}', 0);
    }

    #[test]
    #[should_panic(expected = "is_ascii")]
    fn invalid_add_to_char_output_not_ascii() {
        add_to_char('~', 2);
    }

    #[test]
    #[should_panic(expected = "is_ascii")]
    fn invalid_char_diff_c_not_ascii() {
        char_diff('\u{00E9}', 'a');
    }

    #[test]
    #[should_panic(expected = "is_ascii")]
    fn invalid_char_diff_base_not_ascii() {
        char_diff('a', '\u{00E9}');
    }

    #[test]
    #[should_panic(expected = "c must be greater than or equal to base_char")]
    fn invalid_char_diff_c_less_than_base() {
        char_diff('a', 'z');
    }
}
