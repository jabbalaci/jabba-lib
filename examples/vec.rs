use jabba_lib::jvec;

fn main() {
    let array = [6, 7, 8, 10, 15];
    println!("{:?} is sorted: {}", array, jvec::is_sorted(&array));

    let v = vec![6, 7, 8, 10, 15];
    println!("{:?} is sorted: {}", v, jvec::is_sorted(&v));

    println!("---");

    let numbers = [8, 5, 1, 3];
    let sorted = jvec::sorted_ref(&numbers);
    println!("{:?}", sorted);

    let numbers = vec![8, 5, 1, 3];
    let sorted = jvec::sorted_ref(&numbers);
    println!("{:?}", sorted);

    println!("---");

    let mut numbers = [8, 5, 1, 3];
    let sorted = jvec::sorted_copy(&numbers);
    numbers[0] = 99;
    println!("{:?}", numbers);
    println!("{:?}", sorted);
}
