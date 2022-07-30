use jabba_lib::jmath;

fn main() {
    assert_eq!(jmath::is_palindrome(101), true);
    assert_eq!(jmath::is_prime(97), true);
    assert_eq!(jmath::get_divisors(28), [1, 2, 4, 7, 14, 28]);
    assert_eq!(jmath::factorial(5), 120);
    assert_eq!(jmath::factorial_bigint(33).to_string(), "8683317618811886495518194401280000000");
}
