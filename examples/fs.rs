use jabba_lib::jfs;

fn main() {
    let lines = jfs::readlines("Cargo.toml").unwrap();
    println!("Number of lines in Cargo.toml: {}", lines.len());
    println!("---");
    let content = jfs::read("Cargo.toml").unwrap();
    println!("Number of characters in Cargo.toml: {}", content.len());
}
