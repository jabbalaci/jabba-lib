use jabba_lib::jmath;

fn main() {
    let number = 101;
    println!("{} is palindrome: {}", number, jmath::is_palindrome(number));
    let number = 1977;
    println!("{} is palindrome: {}", number, jmath::is_palindrome(number));

    println!("{} is prime: {}", number, jmath::is_prime(number as u64));

    println!("The first ten prime numbers:");
    let mut primes = jmath::Primes::new();
    for _ in 0..10 {
        println!("{}", primes.next().unwrap());
    }
    println!("---");
    let v = jmath::get_primes_below(12);
    println!("{:?}", v);
}
