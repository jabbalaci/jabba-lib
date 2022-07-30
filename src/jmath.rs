//! math

use crate::jvec;
use num_bigint::BigInt;

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
pub fn is_palindrome(number: i32) -> bool {
    let v = digits(number as u64);
    jvec::is_palindrome(&v)
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
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i: u64 = 3;
    let maxi: f64 = (n as f64).sqrt() + 1.0;
    while (i as f64) <= maxi {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

//-------------------------------------

#[derive(Debug)]
pub struct Primes {
    value: u64,
}

impl Primes {
    pub fn new() -> Primes {
        Primes { value: 1 }
    }
}

impl Iterator for Primes {
    type Item = u64;

    // not efficient
    fn next(&mut self) -> Option<u64> {
        loop {
            self.value += if self.value >= 3 { 2 } else { 1 };
            if is_prime(self.value) {
                return Some(self.value);
            }
        }
    }
}

//-------------------------------------

/// Returns the prime divisors of the given number.
///
/// # Examples
///
/// ```
/// let number = 13195;
/// let answer = jabba_lib::jmath::get_prime_divisors(number);
///
/// assert_eq!(answer, vec![5, 7, 13, 29]);
/// ```
pub fn get_prime_divisors(number: u64) -> Vec<u64> {
    let mut result = vec![];

    let mut n = number;
    let mut primes = Primes::new();
    while n != 1 {
        let prime = primes.next().unwrap();
        while n % prime == 0 {
            n /= prime;
            result.push(prime);
        }
    }

    result
}

/// Returns the digits of the given number.
///
/// Similar to Julia's `digits()` function.
///
/// # Examples
///
/// ```
/// let number = 1977;
/// let answer = jabba_lib::jmath::digits(number);
///
/// assert_eq!(answer, vec![1, 9, 7, 7]);
/// ```
pub fn digits(number: u64) -> Vec<i32> {
    if number == 0 {
        return vec![0];
    }
    // else
    let mut result = vec![];
    let mut n = number;

    while n > 0 {
        let digit = n % 10;
        result.push(digit as i32);
        n /= 10;
    }
    result.reverse();

    result
}

/// Returns the decimal digits inside a string.
///
/// Non-digit characters are discarded.
///
/// # Examples
///
/// ```
/// assert_eq!(jabba_lib::jmath::digits_from_str("1977"), vec![1, 9, 7, 7]);
/// assert_eq!(jabba_lib::jmath::digits_from_str("19abc77"), vec![1, 9, 7, 7]);
/// assert_eq!(jabba_lib::jmath::digits_from_str("ee:ff:90:ac:de"), vec![9, 0]);
/// ```
pub fn digits_from_str(s: &str) -> Vec<i32> {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| char::to_digit(c, 10).unwrap() as i32)
        .collect::<Vec<_>>()
}

/// Returns all the primes below the given number.
///
/// The method uses Aristotle's sieve algorithm.
///
/// # Examples
///
/// ```
/// let primes = jabba_lib::jmath::get_primes_below(10);
///
/// assert_eq!(primes, vec![2, 3, 5, 7]);
/// ```
pub fn get_primes_below(size: usize) -> Vec<usize> {
    let mut v = vec![true; size];
    v[0] = false; // 0 is not prime
    v[1] = false; // 1 is not prime

    for i in 2..size {
        if v[i] {
            for pos in (i + i..size).step_by(i) {
                v[pos] = false;
            }
        }
    }

    let mut result = vec![];
    for (i, value) in v.into_iter().enumerate() {
        if value {
            result.push(i);
        }
    }

    result
}

/// Returns the divisors of the given number.
///
/// # Examples
///
/// ```
/// let number = 28;
/// let answer = jabba_lib::jmath::get_divisors(number);
///
/// assert_eq!(answer, vec![1, 2, 4, 7, 14, 28]);
/// ```
pub fn get_divisors(number: u64) -> Vec<u64> {
    assert!(number > 0);

    let mut result = vec![1];

    let half = number / 2;
    for i in 2..half + 1 {
        if number % i == 0 {
            result.push(i);
        }
    }

    if number > 1 {
        result.push(number);
    }

    result
}

/// Returns the proper divisors of the given number `n`.
///
/// Proper divisors: numbers less than `n` which divide evenly into `n`.
///
/// # Examples
///
/// ```
/// let number = 28;
/// let answer = jabba_lib::jmath::get_proper_divisors(number);
///
/// assert_eq!(answer, vec![1, 2, 4, 7, 14]);
/// ```
pub fn get_proper_divisors(number: u64) -> Vec<u64> {
    let mut result = get_divisors(number);
    result.pop();
    result
}

/// Returns the Collatz sequence of the given number.
///
/// # Examples
///
/// ```
/// let number = 13;
/// let answer = jabba_lib::jmath::get_collatz_sequence(number);
///
/// assert_eq!(answer, vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
/// ```
pub fn get_collatz_sequence(number: u64) -> Vec<u64> {
    assert!(number > 0);

    let mut n = number;
    let mut result = vec![n];

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        result.push(n);
    }
    result
}

/// Returns the factorial of the given number.
///
/// # Examples
///
/// ```
/// let number = 5;
/// let answer = jabba_lib::jmath::factorial(number);
///
/// assert_eq!(answer, 5 * 4 * 3 * 2 * 1);
/// ```
pub fn factorial(n: u128) -> u128 {
    let mut result = 1;
    for i in 2..n + 1 {
        result *= i;
    }
    result
}

/// Returns the factorial of the given number as a BigInt.
///
/// # Examples
///
/// ```
/// assert_eq!(jabba_lib::jmath::factorial_bigint(33).to_string(), "8683317618811886495518194401280000000");
/// assert_eq!(jabba_lib::jmath::factorial_bigint(10).to_string(), jabba_lib::jmath::factorial(10).to_string());
/// ```
pub fn factorial_bigint(n: u128) -> BigInt {
    let mut result = BigInt::from(1);
    for i in 2..n + 1 {
        result *= i;
    }
    result
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert!(is_palindrome(0));
        assert!(is_palindrome(1));
        assert!(is_palindrome(11));
        assert!(is_palindrome(101));
        assert!(is_palindrome(123454321));
        //
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(25), false);
        assert_eq!(is_palindrome(2022), false);
    }

    #[test]
    fn is_prime_test1() {
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

    #[test]
    fn generate_primes_below_100() {
        let numbers = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        let mut primes = Primes::new();
        let result: Vec<u64> = (0..)
            .map(|_| primes.next().unwrap())
            .take_while(|&x| x < 100)
            .collect();
        assert_eq!(result, numbers);
    }

    #[test]
    fn get_prime_divisors_test() {
        assert_eq!(get_prime_divisors(4), [2, 2]);
        assert_eq!(get_prime_divisors(3), [3]);
        assert_eq!(get_prime_divisors(2), [2]);
        assert_eq!(get_prime_divisors(13195), [5, 7, 13, 29]);
    }

    #[test]
    fn digits_test() {
        assert_eq!(digits(123), [1, 2, 3]);
        assert_eq!(digits(1977), [1, 9, 7, 7]);
        assert_eq!(digits(12), [1, 2]);
        assert_eq!(digits(1), [1]);
        assert_eq!(digits(0), [0]);
        assert_eq!(digits(10), [1, 0]);
        assert_eq!(digits(2022), [2, 0, 2, 2]);
    }

    #[test]
    fn digits_from_str_test() {
        assert_eq!(digits_from_str("123"), [1, 2, 3]);
        assert_eq!(digits_from_str("1977"), [1, 9, 7, 7]);
        assert_eq!(digits_from_str("12"), [1, 2]);
        assert_eq!(digits_from_str("1"), [1]);
        assert_eq!(digits_from_str("0"), [0]);
        assert_eq!(digits_from_str("10"), [1, 0]);
        assert_eq!(digits_from_str("2022"), [2, 0, 2, 2]);
        //
        assert_eq!(digits_from_str("202abc2"), [2, 0, 2, 2]);
        assert_eq!(digits_from_str("abc"), []);
        assert_eq!(digits_from_str("aa:bb:42:ee:ff"), [4, 2]);
    }

    #[test]
    fn get_primes_below_test() {
        assert_eq!(get_primes_below(2), []);
        assert_eq!(get_primes_below(3), [2]);
        assert_eq!(get_primes_below(4), [2, 3]);
        assert_eq!(get_primes_below(10), [2, 3, 5, 7]);
        assert_eq!(get_primes_below(13), [2, 3, 5, 7, 11]);
        assert_eq!(get_primes_below(14), [2, 3, 5, 7, 11, 13]);
        //
        assert_eq!(get_primes_below(100).len(), 25);
    }

    #[test]
    fn get_divisors_test() {
        assert_eq!(get_divisors(1), [1]);
        assert_eq!(get_divisors(3), [1, 3]);
        assert_eq!(get_divisors(6), [1, 2, 3, 6]);
        assert_eq!(get_divisors(10), [1, 2, 5, 10]);
        assert_eq!(get_divisors(15), [1, 3, 5, 15]);
        assert_eq!(get_divisors(21), [1, 3, 7, 21]);
        assert_eq!(get_divisors(28), [1, 2, 4, 7, 14, 28]);
    }

    #[test]
    fn get_proper_divisors_test() {
        assert_eq!(get_proper_divisors(1), []);
        assert_eq!(get_proper_divisors(3), [1]);
        assert_eq!(get_proper_divisors(6), [1, 2, 3]);
        assert_eq!(get_proper_divisors(10), [1, 2, 5]);
        assert_eq!(get_proper_divisors(15), [1, 3, 5]);
        assert_eq!(get_proper_divisors(21), [1, 3, 7]);
        assert_eq!(get_proper_divisors(28), [1, 2, 4, 7, 14]);
    }

    #[test]
    fn get_collatz_sequence_test() {
        assert_eq!(get_collatz_sequence(1), [1]);
        assert_eq!(
            get_collatz_sequence(13),
            [13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
    }

    #[test]
    fn factorial_test() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn factorial_bigint_test() {
        let numbers = [2, 3, 5, 7, 11, 13];
        for &n in numbers.iter() {
            assert_eq!(factorial(n).to_string(), factorial_bigint(n).to_string());
        }
    }
}
