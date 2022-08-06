//! jpy

/// Negates a logical expression.
///
/// If you miss `not` from Python.
///
/// # Examples
///
/// ```
/// use jabba_lib::jpy::not;
///
/// assert_eq!(not(true), false);
/// assert_eq!(not(false), true);
/// ```
pub fn not(v: bool) -> bool {
    !v
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_test() {
        assert_eq!(not(true), false);
        assert_eq!(not(false), true);
        assert_eq!(not(1 == 2), true);
        assert_eq!(not(5 < 6), false);
    }
}
