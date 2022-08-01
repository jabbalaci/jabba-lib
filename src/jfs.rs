//! file system

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter};

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

/// Opens a file for buffered reading.
///
/// Useful if you want to read a file line by line.
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

    Ok(BufReader::new(file))
}

/// Opens a file for buffered writing.
///
/// Useful if you want to write to a file line by line.
///
/// It is similar to Python's `f = open("text.txt", "w")`.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use std::io::Write;
///
/// let fname = "out.20220801a.txt";
/// let names = vec!["Alan", "Bob", "Carol"];
/// let mut f = jabba_lib::jfs::open_for_write(fname).unwrap();
/// f.write(b"# it is safe to delete this file\n").unwrap();
/// for name in names.iter() {
///     f.write(name.as_bytes()).unwrap();
///     f.write(b"\n").unwrap();
/// }
/// fs::remove_file(fname).unwrap();
/// ```
///
/// In Python, it would look like this:
///
/// ```python
/// names = ["Alan", "Bob", "Carol"]
/// f = open("out.20220801a.txt", "w")
/// for name in names:
///     f.write(name + "\n")
///     # or:
///     # print(name, file=f)
/// ```
pub fn open_for_write(fname: &str) -> io::Result<BufWriter<File>> {
    let file = File::create(fname)?;

    Ok(BufWriter::new(file))
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

    #[test]
    fn open_for_write_test1() {
        use std::io::Write;

        let fname = "out.20220801b.txt";
        let text = "something";
        {
            let mut f = open_for_write(fname).unwrap();
            f.write(text.as_bytes()).unwrap();
            f.write(b"\n").unwrap();
        }
        let content = read(fname).unwrap();
        assert_eq!(content, "something\n");

        fs::remove_file(fname).unwrap();
    }
}
