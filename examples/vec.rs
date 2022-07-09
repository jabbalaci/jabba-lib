use jabba_lib::jvec;

fn main() {
    let v = vec![6, 7, 8, 10, 15];
    println!("{:?} is sorted: {}", v, jvec::is_sorted(&v));
}
