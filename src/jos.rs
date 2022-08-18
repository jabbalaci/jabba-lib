//! connection to the operating system

use std::{self, env};

/// Returns the short name of the underlying operating system.
///
/// Posible values are listed here: [https://doc.rust-lang.org/std/env/consts/constant.OS.html](https://doc.rust-lang.org/std/env/consts/constant.OS.html)
///
/// # Examples
///
/// ```
/// let name = jabba_lib::jos::get_operating_system_name();
/// ```
pub fn get_operating_system_name() -> &'static str {
    env::consts::OS
}

/// Returns `true` if the operating system is Linux.
///
/// # Examples
///
/// ```
/// let are_we_on_linux = jabba_lib::jos::is_linux();
/// ```
pub fn is_linux() -> bool {
    get_operating_system_name() == "linux"
}

/// Returns `true` if the operating system is Windows.
///
/// # Examples
///
/// ```
/// let are_we_on_windows = jabba_lib::jos::is_windows();
/// ```
pub fn is_windows() -> bool {
    get_operating_system_name() == "windows"
}

/// Returns an estimate of the default amount of parallelism a program should use.
///
/// In case of error, it returns 1.
///
/// Error can happen in the following cases (and thus, 1 is returned):
/// - If the amount of parallelism is not known for the target platform.
/// - If the program lacks permission to query the amount of parallelism made available to it.
///
/// # Examples
///
/// ```
/// let value = jabba_lib::jos::get_available_parallelism();
/// assert!(value >= 1);
/// ```
pub fn get_available_parallelism() -> usize {
    match std::thread::available_parallelism() {
        Ok(value) => value.into(),
        _ => 1,
    }
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_operating_system_name_test() {
        assert!(get_operating_system_name().len() > 0);
    }

    #[test]
    fn is_linux_test() {
        assert!(is_linux() != is_windows());
    }

    #[test]
    fn is_windows_test() {
        assert!(is_windows() != is_linux());
    }

    #[test]
    fn get_available_parallelism_test() {
        let value = get_available_parallelism();
        assert!(value >= 1);
    }
}
