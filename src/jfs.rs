//! file system

use std::fs::{self, File};
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

/// Reads the content of a file and returns it in a single string.
///
/// It is similar to Python's `f.read()`, where `f` is a file handler.
///
/// # Examples
///
/// ```
/// let content: String = jabba_lib::jfs::read("Cargo.toml").unwrap();
/// ```
pub fn read(fname: &str) -> io::Result<String> {
    fs::read_to_string(fname)
}

/// Opens a file for reading.
///
/// With this, you can read a file line by line.
///
/// It is similar to Python's `f = open("text.txt")`.
///
/// # Examples
///
/// ```
/// use std::io::BufRead;
///
/// let f = jabba_lib::jfs::open("Cargo.toml").unwrap();
/// for line in f.lines() {
///     let line = line.unwrap();
///     println!("{}", line);
/// }
/// ```
///
/// In Python, it would look like this:
///
/// ```python
/// f = open("Cargo.toml")
/// for line in f:
///     line = line.rstrip("\n")
///     print(line)
/// ```
pub fn open(fname: &str) -> io::Result<BufReader<File>> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readlines_test1() {
        let fname = "Cargo.toml";
        let lines = readlines(fname).unwrap();
        assert_eq!(lines.is_empty(), false);
    }

    #[test]
    fn read_test1() {
        let fname = "Cargo.toml";
        let content = read(fname).unwrap();
        assert_eq!(content.is_empty(), false);
    }

    #[test]
    fn open_test1() {
        let fname = "Cargo.toml";
        let f = open(fname).unwrap();
        for line in f.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    }
}
