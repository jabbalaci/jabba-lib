//! # permutation
//!
//! Generate the lexicographically next permutation of a sequence
//! of elements.
//!
//! Pseudo-code:
//!
//! 1. Find the largest index `k` such that `a[k] < a[k + 1]`.
//!    If no such index exists, the permutation is the last permutation.
//! 2. Find the largest index `l` such that `a[k] < a[l]`.
//!    Since `k + 1` is such an index, `l` is well-defined and satisfies `k < l`.
//! 3. Swap `a[k]` with `a[l]`.
//! 4. Reverse the sequence from `a[k + 1]` up to end including the final element.

/// Generates the lexicographically next permutation of an array / vector.
///
/// Returns `false` if the permutation is the last permutation.
///
/// The sequence is modified in place.
///
/// # Examples
///
/// ```
/// let mut v = ['a', 'b', 'c'];
///
/// jabba_lib::jpermutation::lexicographically_next_permutation(&mut v);
/// assert_eq!(v, ['a', 'c', 'b']);
/// jabba_lib::jpermutation::lexicographically_next_permutation(&mut v);
/// assert_eq!(v, ['b', 'a', 'c']);
/// jabba_lib::jpermutation::lexicographically_next_permutation(&mut v);
/// assert_eq!(v, ['b', 'c', 'a']);
pub fn lexicographically_next_permutation<T: PartialOrd + Copy>(a: &mut [T]) -> bool {
    let mut i = a.len() - 2;
    loop {
        if a[i] < a[i + 1] {
            break;
        }
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    // else, if a[i] < a[i + 1]
    let mut j = a.len() - 1;
    while !(a[j] > a[i]) {
        j -= 1;
    }
    (a[i], a[j]) = (a[j], a[i]); // swap
    reverse(a, i + 1, a.len() - 1); // reverse the elements from position i+1 until the end
    true
}

/**********
  private
***********/

fn reverse<T: Copy>(a: &mut [T], i: usize, j: usize) {
    let mut i = i;
    let mut j = j;
    while i < j {
        (a[i], a[j]) = (a[j], a[i]); // swap
                                     //
        i += 1;
        j -= 1;
    }
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use lexicographically_next_permutation as next_perm;

    #[test]
    fn lexicographically_next_permutation_test1() {
        let mut v = ['a', 'b', 'c'];
        next_perm(&mut v);
        assert_eq!(v, ['a', 'c', 'b']);
        next_perm(&mut v);
        assert_eq!(v, ['b', 'a', 'c']);
        next_perm(&mut v);
        assert_eq!(v, ['b', 'c', 'a']);
        next_perm(&mut v);
        assert_eq!(v, ['c', 'a', 'b']);
        next_perm(&mut v);
        assert_eq!(v, ['c', 'b', 'a']);
        let status = next_perm(&mut v);
        assert_eq!(status, false);
    }

    #[test]
    fn lexicographically_next_permutation_test2() {
        let mut v = vec!['C', 'A', 'D', 'B'];
        next_perm(&mut v);
        assert_eq!(v, vec!['C', 'B', 'A', 'D']);
        //
        let mut v = [1, 2, 9, 6, 5];
        next_perm(&mut v);
        assert_eq!(v, [1, 5, 2, 6, 9]);
    }
}
