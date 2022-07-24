//! clipboard (supported platforms: Linux (with X server), Windows)

use crate::jos;
use which::which;

// see https://rust-lang-nursery.github.io/rust-cookbook/os/external.html

/// Checks if the current platform is supported.
/// If not supported, then it panics.
///
/// Supported platforms: Linux (with X server), Windows.
///
/// Under Linux you must have the program `xsel` installed.
/// You can install it with your package manager.
///
/// Under Linux, the text is pasted on both clipboards (to "primary" and "clipboard").
///
/// # Examples
///
/// ```
/// jabba_lib::jclipboard::check();  // verify if your platform is supported
/// ```
pub fn check() {
    if !jos::is_linux() && !jos::is_windows() {
        let platform = jos::get_operating_system_name();
        panic!("Error: unknown platform {}", platform);
    }
    // else, if linux or windows:

    if jos::is_windows() {
        return;
    }
    // else, if linux
    let command = "xsel";
    let result = which(command);
    if result.is_err() {
        panic!("Error: the command {} was not found", command);
    }
}

/// Puts the given string on the clipboard.
///
/// Supported platforms: Linux (with X server), Windows.
///
/// Under Linux you must have the program `xsel` installed.
/// You can install it with your package manager.
///
/// Under Linux, the text is pasted on both clipboards (to "primary" and "clipboard").
///
/// # Examples
///
/// ```
/// let backup = jabba_lib::jclipboard::get_text().unwrap();
/// let text = "hello";
///
/// jabba_lib::jclipboard::check();  // verify if your platform is supported
///
/// jabba_lib::jclipboard::set_text(text).unwrap();
/// println!("The text {:?} was pasted on the clipboard", text);
///
/// let read = jabba_lib::jclipboard::get_text().unwrap();
/// println!("Contents of the clipboard: {:?}", read);
///
/// assert_eq!(read, text);
/// jabba_lib::jclipboard::set_text(&backup).unwrap();
/// ```
pub fn set_text(text: &str) -> Result<(), &'static str> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            set_text_windows(text)?;
            Ok(())
        } else if #[cfg(unix)] {
            let result = set_text_linux(text);
            match result {
                Ok(_) => Ok(()),
                Err(_) => Err("Error: cannot write to clipboard"),
            }
        } else {
            let platform = jos::get_operating_system_name();
            panic!("Error: unknown platform {}", platform);
        }
    }
}

/// Reads the contents of the clipboard.
///
/// Supported platforms: Linux (with X server), Windows.
///
/// Under Linux you must have the program `xsel` installed.
/// You can install it with your package manager.
///
/// # Examples
///
/// ```
/// let backup = jabba_lib::jclipboard::get_text().unwrap();
/// let text = "hello";
///
/// jabba_lib::jclipboard::check();  // verify if your platform is supported
///
/// jabba_lib::jclipboard::set_text(text).unwrap();
/// println!("The text {:?} was pasted on the clipboard", text);
///
/// let read = jabba_lib::jclipboard::get_text().unwrap();
/// println!("Contents of the clipboard: {:?}", read);
///
/// assert_eq!(read, text);
/// jabba_lib::jclipboard::set_text(&backup).unwrap();
/// ```
pub fn get_text() -> Result<String, &'static str> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            let text = get_text_windows()?;
            Ok(text)
        } else if #[cfg(unix)] {
            let result = get_text_linux();
            match result {
                Ok(text) => Ok(text),
                Err(_) => Err("Error: cannot read from clipboard"),
            }
        } else {
            let platform = jos::get_operating_system_name();
            panic!("Error: unknown platform {}", platform);
        }
    }
}

////////////////////////////////
// Windows-specific functions //
////////////////////////////////

#[cfg(windows)]
use clipboard_win::formats::Unicode;
#[cfg(windows)]
use clipboard_win::{Clipboard, Getter, Setter};

#[cfg(windows)]
fn set_text_windows(text: &str) -> Result<(), &'static str> {
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    let result = Unicode.write_clipboard(&text);
    match result {
        Ok(_) => Ok(()),
        _ => Err("cannot write to clipboard"),
    }
}

#[cfg(windows)]
fn get_text_windows() -> Result<String, &'static str> {
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    let mut output = String::new();
    let result = Unicode.read_clipboard(&mut output);
    match result {
        Ok(_) => Ok(output),
        _ => Err("cannot read clipboard"),
    }
}

//////////////////////////////
// Linux-specific functions //
//////////////////////////////

#[cfg(unix)]
use std::error::Error;
#[cfg(unix)]
use std::process::{Command, Stdio};

#[cfg(unix)]
fn set_text_linux(text: &str) -> Result<(), Box<dyn Error>> {
    set_text_with_xsel(text, "-pi")?; // primary
    set_text_with_xsel(text, "-bi")?; // clipboard
    Ok(())
}

#[cfg(unix)]
fn set_text_with_xsel(text: &str, which_clipboard: &str) -> Result<(), Box<dyn Error>> {
    let mut echo_output_child = Command::new("echo")
        .arg("-n")
        .arg(text)
        .stdout(Stdio::piped())
        .spawn()?;

    echo_output_child.wait()?;

    if let Some(echo_output) = echo_output_child.stdout.take() {
        let mut xsel_output_child = Command::new("xsel")
            .arg(which_clipboard) // difference here
            .stdin(echo_output)
            .spawn()?;

        xsel_output_child.wait()?;
    }

    Ok(())
}

#[cfg(unix)]
fn get_text_linux() -> Result<String, Box<dyn Error>> {
    let output_child = Command::new("xsel")
        .arg("-bo")
        .stdout(Stdio::piped())
        .spawn()?;

    let prg_stdout = output_child.wait_with_output()?;
    Ok(String::from_utf8(prg_stdout.stdout)?)
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test() {
        assert_eq!(check(), ());
    }

    #[test]
    fn set_text_and_get_text_test() {
        check();
        let backup = get_text().unwrap();
        //
        for &s in &["", "a", "aa", "hello", "rust is cool", "Éva"] {
            set_text(s).unwrap();
            assert_eq!(get_text().unwrap(), s);
        }
        //
        set_text(&backup).unwrap();
        assert_eq!(get_text().unwrap(), backup);
    }
}
