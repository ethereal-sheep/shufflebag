//! A shuffle bag implementation in `rust`.
//!
//! The bag allows for a pseudo random drawing of its elements
//! by assigning a random float64 value to each value and storing
//! it in a maximal `BinaryHeap<T>`.
//!
//! Therefore, `push()` and `pop()` complexities follow those of the
//! `std::collections::BinaryHeap`.
#![warn(rust_2018_idioms)]

use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct ShuffleBag<T> {
    bag: BinaryHeap<ShufflePair<T>>,
    rng: StdRng,
}

/// A shuffle bag implementation in `rust`.
///
/// The bag allows for a pseudo random drawing of its elements
/// by assigning a random float64 value to each value and storing
/// it in a maximal `BinaryHeap<T>`.
impl<T> ShuffleBag<T> {
    /// Returns the number of elements in the `ShuffleBag`.
    ///
    /// # Examples
    ///
    /// ```
    /// use shufflebag::ShuffleBag;
    /// let mut bag: ShuffleBag<i32> = ShuffleBag::new();
    ///
    /// assert_eq!(bag.len(), 0);
    /// bag.push(1);
    /// assert_eq!(bag.len(), 1);
    /// bag.push(1);
    /// assert_eq!(bag.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.bag.len()
    }

    /// Returns `true` if the bag contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use shufflebag::ShuffleBag;
    /// let mut bag: ShuffleBag<i32> = ShuffleBag::new();
    ///
    /// assert!(bag.is_empty());
    /// bag.push(1);
    /// assert!(!bag.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bag.is_empty()
    }

    /// Creates an empty `ShuffleBag`.
    ///
    /// The shufflebag is seeded with `StdRng::from_entropy()`
    ///
    /// # Example
    ///
    /// ```
    /// use shufflebag::ShuffleBag;
    /// let bag: ShuffleBag<i32> = ShuffleBag::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            bag: BinaryHeap::new(),
            rng: StdRng::from_entropy(),
        }
    }

    /// Creates a `ShuffleBag` from an array.
    ///
    /// # Example
    ///
    /// ```
    /// use shufflebag::ShuffleBag;
    /// let bag: ShuffleBag<i32> = ShuffleBag::from([1,2,3]);
    /// assert_eq!(bag.len(), 3);
    /// ```
    #[inline]
    pub fn from<const N: usize>(values: [T; N]) -> Self {
        let mut ret = Self::new();

        for value in values {
            ret.push(value);
        }

        ret
    }

    /// Creates an empty `ShuffleBag` with an StdRng::Seed
    ///
    /// See [`from_seed`](https://rust-random.github.io/rand/rand/trait.SeedableRng.html#tymethod.from_seed)
    #[inline]
    pub fn with_seed(seed: <StdRng as SeedableRng>::Seed) -> Self {
        Self {
            bag: BinaryHeap::new(),
            rng: StdRng::from_seed(seed),
        }
    }

    /// Creates an empty `ShuffleBag` with an u64 seed.
    ///
    /// # Example
    ///
    /// ```
    /// use shufflebag::ShuffleBag;
    /// let bag: ShuffleBag<i32> = ShuffleBag::with_u64(42);
    /// ```
    pub fn with_u64(seed: u64) -> Self {
        Self {
            bag: BinaryHeap::new(),
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Pushes an item into the `ShuffleBag`.
    ///
    /// # Example
    ///
    /// ```
    /// use shufflebag::ShuffleBag;
    /// let mut bag: ShuffleBag<i32> = ShuffleBag::new();
    /// bag.push(3);
    /// bag.push(5);
    /// bag.push(1);
    ///
    /// assert_eq!(bag.len(), 3);
    /// ```
    pub fn push(&mut self, value: T) {
        self.bag.push(ShufflePair(value, self.rng.gen()))
    }

    /// Pops an item from the `ShuffleBag`.
    ///
    /// # Example
    ///
    /// ```
    /// use shufflebag::ShuffleBag;
    /// let mut bag: ShuffleBag<i32> = ShuffleBag::from([1,2,3]);
    /// let i = bag.pop();
    /// let j = bag.pop();
    /// let k = bag.pop();
    ///
    /// assert!(i.is_some());
    /// assert!(j.is_some());
    /// assert!(k.is_some());
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.bag.pop().map(|p| p.0)
    }
}

impl<T> Default for ShuffleBag<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct ShufflePair<T>(T, f64);

impl<T> Ord for ShufflePair<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> PartialOrd for ShufflePair<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<T> Eq for ShufflePair<T> {}
impl<T> PartialEq for ShufflePair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn small_bag() {
        let mut b = ShuffleBag::<i32>::new();

        let mut vecs = vec![vec![0; 3]; 3];
        const ITERATIONS: i32 = 1000000;
        const BENCH: i32 = ITERATIONS / 3 + ITERATIONS / 100;

        for _ in 0..ITERATIONS {
            b.push(0);
            b.push(1);
            b.push(2);

            for i in 0..3 {
                let item = b.pop().unwrap();
                vecs[i][item as usize] += 1;
            }
        }
        
        // assert a close to uniform distribution
        for v in &vecs {
            for item in v {
                assert!(*item < BENCH);
            }
        }
    }
    #[test]
    fn large_bag() {
        let mut b = ShuffleBag::<i32>::new();

        const ITERATIONS: usize = 10000;
        const ITEMS: usize = 100;
        const BENCH: usize = ITERATIONS + ITERATIONS / 100usize;
        let mut vecs = vec![vec![0; ITEMS]; ITEMS];

        for _ in 0..ITERATIONS {
            for i in 0..ITEMS {
                b.push(i as i32);
            }

            for i in 0..ITEMS {
                let item = b.pop().unwrap();
                vecs[i][item as usize] += 1;
            }
        }

        // assert a close to uniform distribution
        for v in &vecs {
            let mut count = 0;
            for item in v {
                count += *item;
            }
            assert!(count < BENCH);
        }
    }
    #[test]
    fn normal_bag() {
        let mut b = ShuffleBag::<i32>::new();

        const ITERATIONS: usize = 10000;
        const ITEMS: usize = 5;
        const _BENCH: usize = ITERATIONS + ITERATIONS / 100usize;
        let mut vecs = vec![vec![0; ITEMS + ITEMS]; ITEMS + ITEMS];

        for _ in 0..ITERATIONS {
            for i in 0..ITEMS {
                for j in 0..ITEMS {
                    b.push(i as i32 + j as i32);
                }
            }

            for i in 0..ITEMS + ITEMS {
                let item = b.pop().unwrap();
                vecs[i][item as usize] += 1;
            }
        }

        // assert a close to pyramid distribution
        // for v in &vecs {
        //     // let mut count = 0;
        //     for item in v {
        //         print!("{} ", item);
        //         // count += *item;
        //     }
        //     println!();
        //     // assert!(count < BENCH);
        // }
    }
}
