//! time

use std::thread;
use std::time::Duration;

/// Delay execution for a given number of seconds.
/// Seconds are given as a float (f64).
///
/// Similar to Python's `time.sleep()`.
///
/// # Examples
///
/// ```ignore
/// // wait 0.5 seconds
/// jabba_lib::jtime::sleep(0.5);
/// ```
pub fn sleep(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}
