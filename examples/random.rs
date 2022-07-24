use jabba_lib::jrandom;

fn main() {
    let number = jrandom::randrange(1, 10);
    println!("random number (randrange): {}", number);

    let number = jrandom::randint(1, 100);
    println!("random number (randint): {}", number);

    let mut numbers = vec![1, 2, 3, 4, 5];
    jrandom::shuffle(&mut numbers);
    println!("shuffled: {:?}", numbers);

    let mut numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    jrandom::shuffle(&mut numbers);
    println!("shuffled: {:?}", numbers);

    let number = jrandom::random();
    println!("random float from [0.0, 1.0) : {}", number);

    let numbers = vec![1, 2, 3, 4, 5, 6];
    let roll = jrandom::choice(&numbers).unwrap();
    println!("dice roll: {}", roll);
}
