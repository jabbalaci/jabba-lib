//! vector

/// Returns `true` if the array / vector is sorted.
///
/// # Examples
///
/// ```
/// let vec_yes = vec![1, 2, 3, 4];
/// let vec_no = vec![1, 4, 3, 2];
/// let array_yes = [1, 4, 8, 8];
///
/// assert_eq!(jabba_lib::jvec::is_sorted(&vec_yes), true);
/// assert_eq!(jabba_lib::jvec::is_sorted(&vec_no), false);
/// assert_eq!(jabba_lib::jvec::is_sorted(&array_yes), true);
/// ```
pub fn is_sorted<T: PartialOrd>(v: &[T]) -> bool {
    if v.is_empty() {
        return true;
    }

    for i in 0..v.len() - 1 {
        if &v[i] > &v[i + 1] {
            return false;
        }
    }
    true
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_sorted_test1() {
        assert!(is_sorted(&Vec::<i32>::new()));
        assert!(is_sorted(&vec![1]));
        assert!(is_sorted(&vec![1, 1]));
        assert!(is_sorted(&vec![1, 2, 3]));
        assert!(is_sorted(&vec![1, 2, 3, 3]));
        assert!(is_sorted(&vec![1, 2, 3, 4, 5]));
        assert!(is_sorted(&vec![1, 2, 3, 4, 5, 5, 5, 5]));
    }

    #[test]
    fn is_sorted_test2() {
        assert_eq!(is_sorted(&vec![2, 1]), false);
        assert_eq!(is_sorted(&vec![1, 2, 3, 4, 3]), false);
        assert_eq!(is_sorted(&vec![1, 2, 3, 2, 5]), false);
        assert_eq!(is_sorted(&vec![1, 2, 1, 4, 5]), false);
        assert_eq!(is_sorted(&vec![1, 0, 3, 4, 5]), false);
        assert_eq!(is_sorted(&vec![3, 2, 3, 4, 5]), false);
    }

    #[test]
    fn is_sorted_test3() {
        assert!(is_sorted(&Vec::<&str>::new()));
        assert!(is_sorted(&vec!["aa"]));
        assert!(is_sorted(&vec!["aa", "aa"]));
        assert!(is_sorted(&vec!["aa", "bb", "cc"]));
        //
        assert_eq!(is_sorted(&vec!["aa", "cc", "bb"]), false);
        assert_eq!(is_sorted(&vec!["aa", "bb", "cc", "bb"]), false);
    }
}
