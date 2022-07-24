//! math

use crate::jvec;

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
}
