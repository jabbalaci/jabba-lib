use jabba_lib::jfs;
use std::io::BufRead;

fn main() {
    let lines = jfs::readlines("Cargo.toml").unwrap();
    println!("Number of lines in Cargo.toml: {}", lines.len());
    println!("---");
    let content = jfs::read("Cargo.toml").unwrap();
    println!("Number of characters in Cargo.toml: {}", content.len());
    println!("---");
    let f = jfs::open("Cargo.toml").unwrap();
    for line in f.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
