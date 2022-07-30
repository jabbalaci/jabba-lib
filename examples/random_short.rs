use jabba_lib::jrandom;

fn main() {
    let number = jrandom::randrange(1, 10); // 10 is excluded
    println!("random number from [1, 10): {}", number);

    let number = jrandom::randint(1, 100); // 100 is included
    println!("random number from [1, 100]: {}", number);

    let mut numbers = vec![1, 2, 3, 4, 5];
    jrandom::shuffle(&mut numbers);
    println!("shuffled: {:?}", numbers); // could be [3, 5, 1, 4, 2]
}
