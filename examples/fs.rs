use jabba_lib::jfs;
use std::io::{BufRead, Write};

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
    println!("---");
    let fname = "out.20220801c.txt";
    let names = vec!["Alan", "Bob", "Carol"];
    let mut f = jfs::open_for_write(fname).unwrap();
    f.write(b"# it is safe to delete this file\n").unwrap();
    for name in names.iter() {
        f.write(name.as_bytes()).unwrap();
        f.write(b"\n").unwrap();
    }
}
