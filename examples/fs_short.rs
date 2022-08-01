use jabba_lib::jfs;
use std::io::BufRead;

fn main() {
    let f = jfs::open("Cargo.toml").unwrap();
    for line in f.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
