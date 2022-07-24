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
        if v[i] > v[i + 1] {
            return false;
        }
    }
    true
}

/// Returns a sorted vector (of references) of the input array / vector.
///
/// The returned vector contains references.
///
/// # Examples
///
/// ```
/// let array = [8, 2, 9, 3];
/// assert_eq!(jabba_lib::jvec::sorted_ref(&array), vec![&2, &3, &8, &9]);
///
/// let v = vec![8, 2, 9, 3];
/// assert_eq!(jabba_lib::jvec::sorted_ref(&v), vec![&2, &3, &8, &9]);
/// ```
pub fn sorted_ref<T: Ord>(v: &[T]) -> Vec<&T> {
    let mut copy = v.iter().collect::<Vec<_>>();
    copy.sort();
    copy
}

/// Returns a sorted copy (a vector) of the input array / vector.
///
/// The returned vector contains copies.
///
/// # Examples
///
/// ```
/// let array = [8, 2, 9, 3];
/// assert_eq!(jabba_lib::jvec::sorted_copy(&array), vec![2, 3, 8, 9]);
///
/// let v = vec![8, 2, 9, 3];
/// assert_eq!(jabba_lib::jvec::sorted_copy(&v), vec![2, 3, 8, 9]);
///
/// let mut v = vec![6, 1, 9, 2, 0];
/// let sorted = jabba_lib::jvec::sorted_copy(&v);
/// v[0] = 99;
/// assert_eq!(v, vec![99, 1, 9, 2, 0]);
/// assert_eq!(sorted, vec![0, 1, 2, 6, 9]);
/// ```
pub fn sorted_copy<T: Ord + Clone>(v: &[T]) -> Vec<T> {
    let mut copy: Vec<T> = v.iter().map(|x| (*x).clone()).collect();
    copy.sort();
    copy
}

/// Returns `true` if the array / vector is palindrome.
///
/// # Examples
///
/// ```
/// let array1 = [8, 2, 2, 8];
/// assert_eq!(jabba_lib::jvec::is_palindrome(&array1), true);
///
/// let array2 = [2, 0, 2, 2];
/// assert_eq!(jabba_lib::jvec::is_palindrome(&array2), false);
///
/// let v = vec!["aa", "bb", "aa"];
/// assert_eq!(jabba_lib::jvec::is_palindrome(&v), true);
///
/// ```
pub fn is_palindrome<T: PartialEq>(v: &[T]) -> bool {
    if v.len() < 2 {
        return true;
    }
    // else
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        if v[i] != v[j] {
            return false;
        }
        i += 1;
        j -= 1;
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

    #[test]
    fn sorted_ref_test1() {
        let array = [8, 2, 9, 3];
        assert_eq!(sorted_ref(&array), vec![&2, &3, &8, &9]);

        let v = Vec::<i32>::new();
        assert_eq!(sorted_ref(&v), Vec::<&i32>::new());

        let v = ["bb", "cc", "aa"];
        assert_eq!(sorted_ref(&v), vec![&"aa", &"bb", &"cc"]);
    }

    #[test]
    fn sorted_copy_test1() {
        let mut v = vec![6, 1, 9, 2, 0];
        let sorted = sorted_copy(&v);
        v[0] = 99;
        assert_eq!(v, vec![99, 1, 9, 2, 0]);
        assert_eq!(sorted, vec![0, 1, 2, 6, 9]);

        let array = [8, 2, 9, 3];
        assert_eq!(sorted_copy(&array), vec![2, 3, 8, 9]);

        let v = ["bb", "cc", "aa"];
        assert_eq!(sorted_copy(&v), vec!["aa", "bb", "cc"]);
    }

    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome(&Vec::<i32>::new()), true);
        assert_eq!(is_palindrome(&[0]), true);
        assert_eq!(is_palindrome(&[0, 1]), false);
        assert_eq!(is_palindrome(&[1, 1]), true);
        assert_eq!(is_palindrome(&[1, 0, 1]), true);
        //
        assert_eq!(is_palindrome(&[1, 9, 7, 7]), false);
        assert_eq!(is_palindrome(&[2, 0, 2, 2]), false);
        assert_eq!(is_palindrome(&vec![1, 9, 9, 1]), true);
    }
}
