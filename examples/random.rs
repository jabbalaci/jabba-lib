use jabba_lib::jrandom;

fn main() {
    let number = jrandom::randrange(1, 10);
    println!("random number (randrange): {}", number);

    let number = jrandom::randint(1, 100);
    println!("random number (randint): {}", number);

    let mut numbers = vec![1, 2, 3, 4, 5];
    jrandom::shuffle(&mut numbers);
    println!("shuffled: {:?}", numbers);
}
