use jabba_lib::jpermutation as jp;

fn main() {
    let mut v = ['c', 'a', 'b', 'e', 'd'];
    println!("{:?}", v);
    println!();
    for _ in 0..3 {
        jp::lexicographically_next_permutation(&mut v);
        println!("{:?}", v);
    }

    println!("---");

    let mut v = ['a', 'b', 'c'];
    println!("{:?}", v);
    loop {
        let status = jp::lexicographically_next_permutation(&mut v);
        if !status {
            break;
        }
        println!("{:?}", v);
    }
}
