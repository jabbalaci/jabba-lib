//! file system

use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Reads all the lines of a file and returns them in a vector.
///
/// It is similar to Python's `f.readlines()`, where `f` is a file handler.
///
/// # Examples
///
/// ```
/// let lines: Vec<String> = jabba_lib::jfs::readlines("Cargo.toml").unwrap();
/// ```
pub fn readlines(fname: &str) -> io::Result<Vec<String>> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
