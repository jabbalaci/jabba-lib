//! random

use rand::seq::SliceRandom;
use rand::Rng; // for shuffle

/// Returns a random i32 from range `[lo, hi)`, where `lo` is included
/// and `hi` is excluded.
///
/// Similar to Python's `random.randrange()`.
///
/// # Examples
///
/// ```
/// let number = jabba_lib::jrandom::randrange(1, 6);
///
/// assert!(1 <= number && number < 6);
/// ```
pub fn randrange(lo: i32, hi: i32) -> i32 {
    rand::thread_rng().gen_range(lo..hi)
}

/// Returns a random i32 from range `[lo, hi]`, where both `lo` and `hi` are included.
///
/// Similar to Python's `random.randint()`.
///
/// # Examples
///
/// ```
/// let number = jabba_lib::jrandom::randint(1, 6);
///
/// assert!(1 <= number && number <= 6);
/// ```
pub fn randint(lo: i32, hi: i32) -> i32 {
    rand::thread_rng().gen_range(lo..=hi)
}

/// Shuffles a vector / array in place.
///
/// Similar to Python's `random.shuffle()`.
///
/// # Examples
///
/// ```ignore
/// let mut numbers = vec![1, 2, 3, 4, 5];
/// jabba_lib::jrandom::shuffle(&mut numbers);
/// // now the elements in `numbers` are shuffled
/// // `numbers` could be [4, 2, 3, 1, 5], for instance
/// ```
pub fn shuffle<T>(v: &mut [T]) {
    let mut rng = rand::thread_rng();

    v.shuffle(&mut rng);
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randrange_test1() {
        for _ in 0..10 {
            let value = randrange(1, 2);
            assert!(value == 1);
        }
        //
        for _ in 0..10 {
            let value = randrange(1, 3);
            assert!(value == 1 || value == 2);
        }
        //
        for _ in 0..1000 {
            let value = randrange(1, 10);
            assert!(value >= 1 && value < 10);
        }
    }

    #[test]
    fn randint_test1() {
        for _ in 0..10 {
            let value = randint(1, 1);
            assert!(value == 1);
        }
        //
        for _ in 0..10 {
            let value = randint(1, 2);
            assert!(value >= 1 && value <= 2);
        }
        //
        for _ in 0..1000 {
            let value = randint(1, 10);
            assert!(value >= 1 && value <= 10);
        }
    }

    #[test]
    fn randrange_test2() {
        let mut v = vec![];
        for _ in 0..1000 {
            let value = randrange(1, 10);
            v.push(value);
        }
        v.sort();
        v.dedup();
        for i in 1..=9 {
            assert!(v.contains(&i));
        }
        assert_eq!(v.contains(&0), false);
        assert_eq!(v.contains(&10), false);
        //
        assert!(v[0] == 1);
        assert!(v[v.len() - 1] == 9);
    }

    #[test]
    fn randint_test2() {
        let mut v = vec![];
        for _ in 0..1000 {
            let value = randint(1, 10);
            v.push(value);
        }
        v.sort();
        v.dedup();
        for i in 1..=10 {
            assert!(v.contains(&i));
        }
        assert_eq!(v.contains(&0), false);
        assert_eq!(v.contains(&11), false);
        //
        assert!(v[0] == 1);
        assert!(v[v.len() - 1] == 10);
    }

    #[test]
    fn shuffle_test1() {
        let mut v = vec![];
        for i in 1..=100 {
            v.push(i);
        }
        assert!(v.len() == 100);
        let backup = v.clone();
        //
        shuffle(&mut v);
        assert!(v.len() == backup.len());
        assert!(v != backup);
        v.sort();
        assert!(v == backup);
    }
}
