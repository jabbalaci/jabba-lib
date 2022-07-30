//! spell

use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::jmath;

/*********
  public
**********/

/// Returns the spelled (written out in words) version of the given number.
///
/// Current limitation: the number must be between 0 and 1000 (inclusive).
///
/// # Examples
///
/// ```
/// assert_eq!(jabba_lib::jspell::spell_number(0), "zero");
/// assert_eq!(jabba_lib::jspell::spell_number(115), "one hundred and fifteen");
/// assert_eq!(jabba_lib::jspell::spell_number(342), "three hundred and forty-two");
/// ```
pub fn spell_number(n: u32) -> String {
    assert!(n <= 1000);

    let digits = jmath::digits(n as u64);
    match digits.len() {
        1 => length_1(&digits),
        2 => length_2(&digits),
        3 => length_3(&digits),
        4 => length_4(&digits),
        _ => String::from("unknown"),
    }
}

/**********
  private
***********/

static NUMBERS: Lazy<HashMap<i32, &str>> = Lazy::new(|| {
    HashMap::from([
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100, "one hundred"),
        (1000, "one thousand"),
    ])
});

fn length_1(digits: &[i32]) -> String {
    NUMBERS.get(&digits[0]).unwrap().to_string()
}

fn length_2(digits: &[i32]) -> String {
    let n = digits[0] * 10 + digits[1];
    if digits[0] == 1 {
        NUMBERS.get(&n).unwrap().to_string()
    } else {
        if digits[1] == 0 {
            return NUMBERS.get(&n).unwrap().to_string();
        }
        // else
        let n = digits[0] * 10;
        format!(
            "{}-{}",
            NUMBERS.get(&n).unwrap(),
            NUMBERS.get(&digits[1]).unwrap()
        )
    }
}

fn length_3(digits: &[i32]) -> String {
    let head = digits[0];
    let tail = digits[1] * 10 + digits[2];
    if tail == 0 {
        // "00"
        format!("{} hundred", length_1(&[head]))
    } else if tail < 10 {
        // "01", "02", ..., "09"
        format!("{} hundred and {}", length_1(&[head]), length_1(&[tail]))
    } else {
        let tail_digits = jmath::digits(tail as u64);
        format!(
            "{} hundred and {}",
            length_1(&[head]),
            length_2(&tail_digits)
        )
    }
}

fn length_4(digits: &[i32]) -> String {
    let n = digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3];
    NUMBERS.get(&n).unwrap().to_string()
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spell_number_test1() {
        assert_eq!(spell_number(0), "zero");
        assert_eq!(spell_number(0), "zero");
        assert_eq!(spell_number(1), "one");
        assert_eq!(spell_number(2), "two");
        assert_eq!(spell_number(3), "three");
        assert_eq!(spell_number(4), "four");
        assert_eq!(spell_number(5), "five");
        assert_eq!(spell_number(6), "six");
        assert_eq!(spell_number(7), "seven");
        assert_eq!(spell_number(8), "eight");
        assert_eq!(spell_number(9), "nine");
        assert_eq!(spell_number(10), "ten");
        assert_eq!(spell_number(11), "eleven");
        assert_eq!(spell_number(12), "twelve");
        assert_eq!(spell_number(13), "thirteen");
        assert_eq!(spell_number(14), "fourteen");
        assert_eq!(spell_number(15), "fifteen");
        assert_eq!(spell_number(16), "sixteen");
        assert_eq!(spell_number(17), "seventeen");
        assert_eq!(spell_number(18), "eighteen");
        assert_eq!(spell_number(19), "nineteen");
        assert_eq!(spell_number(20), "twenty");
        assert_eq!(spell_number(30), "thirty");
        assert_eq!(spell_number(40), "forty");
        assert_eq!(spell_number(50), "fifty");
        assert_eq!(spell_number(60), "sixty");
        assert_eq!(spell_number(70), "seventy");
        assert_eq!(spell_number(80), "eighty");
        assert_eq!(spell_number(90), "ninety");
        assert_eq!(spell_number(100), "one hundred");
        assert_eq!(spell_number(1000), "one thousand");
        //
        assert_eq!(spell_number(115), "one hundred and fifteen");
        assert_eq!(spell_number(342), "three hundred and forty-two");
    }

    #[test]
    fn spell_number_test2() {
        let mut sb = String::new();
        for n in 1..1000 + 1 {
            sb.push_str(&spell_number(n));
        }
        sb = sb.replace(" ", "").replace("-", "");
        assert_eq!(sb.len(), 21124);
    }
}
