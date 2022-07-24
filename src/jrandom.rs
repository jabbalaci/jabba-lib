//! random

use rand::seq::SliceRandom;
use rand::Rng;

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
/// ```
/// let mut numbers = vec![1, 2, 3, 4, 5];
/// jabba_lib::jrandom::shuffle(&mut numbers);
/// // now the elements in `numbers` are shuffled
/// // `numbers` could be [4, 2, 3, 1, 5], for instance
/// ```
pub fn shuffle<T>(v: &mut [T]) {
    let mut rng = rand::thread_rng();

    v.shuffle(&mut rng);
}

/// Returns an `f64` from the interval `[0.0, 1.0)`.
///
/// Similar to Python's `random.random()`.
///
/// # Examples
///
/// ```
/// let number = jabba_lib::jrandom::random();
/// // now 0.0 <= number < 1.0
/// ```
pub fn random() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen::<f64>()
}

/// Chooses a random element from an array / vector.
///
/// Returns `None` if the array / vector is empty.
///
/// # Examples
///
/// ```
/// let v = vec![1, 2, 3];
/// let elem: &i32 = jabba_lib::jrandom::choice(&v).unwrap();
/// // elem is now 1 or 2 or 3
/// ```
pub fn choice<T>(v: &[T]) -> Option<&T> {
    if v.is_empty() {
        return None;
    }
    // else
    let idx = randrange(0, v.len() as i32) as usize;
    Some(&v[idx])
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

    #[test]
    fn random_test1() {
        for _ in 0..100 {
            let number = random();
            assert!(number >= 0.0 && number < 1.0)
        }
    }

    #[test]
    fn choice_test1() {
        let v = vec![1, 2, 3];
        for _ in 0..100 {
            let &elem = choice(&v).unwrap();
            assert!(v.contains(&elem));
        }
    }

    #[test]
    fn choice_test2() {
        let empty = Vec::<i32>::new();
        let elem = choice(&empty);
        assert_eq!(elem, None);
    }
}
