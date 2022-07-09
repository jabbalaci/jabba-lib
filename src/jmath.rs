//! math

use crate::jstring;

/// Returns `true` if the given number is palindrome.
///
/// # Examples
///
/// ```
/// let number = 101;
/// let answer = jabba_lib::jmath::is_palindrome(number);
///
/// assert_eq!(answer, true);
/// ```
pub fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    jstring::is_palindrome(&s)
}

/// Returns `true` if the given number is prime.
///
/// Note that this solution is not efficient. If you want to test
/// lots of and/or large numbers, use a more efficient solution.
///
/// # Examples
///
/// ```
/// let number = 23;
/// let answer = jabba_lib::jmath::is_prime(number);
///
/// assert_eq!(answer, true);
/// ```
pub fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i: i64 = 3;
    let maxi: f64 = (n as f64).sqrt() + 1.0;
    while (i as f64) <= maxi {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test1() {
        assert!(is_palindrome(0));
        assert!(is_palindrome(1));
        assert!(is_palindrome(11));
        assert!(is_palindrome(101));
        assert!(is_palindrome(123454321));
    }

    #[test]
    fn is_palindrome_test2() {
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(25), false);
        assert_eq!(is_palindrome(2022), false);
    }

    #[test]
    fn is_prime_test1() {
        assert_eq!(is_prime(-5), false);
        assert_eq!(is_prime(-2), false);
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(97), true);
        assert_eq!(is_prime(100), false);
    }
}
