use jabba_lib::jmath;

fn main() {
    let number = 101;
    println!("{} is palindrome: {}", number, jmath::is_palindrome(number));
    let number = 1977;
    println!("{} is palindrome: {}", number, jmath::is_palindrome(number));

    println!("{} is prime: {}", number, jmath::is_prime(number as i64));
}
