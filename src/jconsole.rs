//! console

use crate::jstring;
use std::io::{self, Write};

/// Flushes stdout.
///
/// # Examples
///
/// ```
/// jabba_lib::jconsole::flush();
/// ```
pub fn flush() {
    io::stdout().flush().unwrap();
}

/// Shows a prompt to the user and reads a line from stdin.
///
/// It is similar to Python's `input()` function.
///
/// # Examples
///
/// ```ignore
/// let name = jabba_lib::jconsole::input("Name: ");
/// // assume you type "Anna" (without quotes) and press Enter
/// assert_eq!(name, "Anna");
/// ```
pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    flush();

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line from stdin");

    jstring::chomp(&mut text);
    text
}
