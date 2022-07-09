use jabba_lib::jfs;

fn main() {
    let lines = jfs::readlines("Cargo.toml").unwrap();
    println!("Number of lines in Cargo.toml: {}", lines.len());
}
