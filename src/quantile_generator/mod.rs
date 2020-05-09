//! Iterators with a given length and a known quantile point
//!
//! This module provides iterators of `N` floats with the value `x` for the quantile `q`, where
//! `N`, `x` and `q` can be directly controlled. The floats are represented by `NotNan<f64>`,
//! because this type implements `Ord`.
//!
//! The rule respected is: `rank_x = ceil(q * (N - 1))`, where `rank_x` is defined as the number of
//! values strictly smaller than `x`. At the extremes, with `q = 0`, `x` is the least returned value
//! and with `q = 1` it is the largest.
//!
//! This module is mainly used to provide test data in order to test the quantile implementations.

mod random;
mod sequential;

use ordered_float::NotNan;
use std::iter::FusedIterator;

/// The main trait representing an iterator of floats
pub trait QuantileGenerator:
    Iterator<Item = NotNan<f64>> + ExactSizeIterator + FusedIterator
{
}

pub use random::RandomGenerator;
pub use sequential::{SequentialGenerator, SequentialOrder};

#[cfg(test)]
mod test {
    use super::*;

    use crate::quantile_to_rank;

    #[test]
    fn median() {
        check_all(0.5, 17., 1);
        check_all(0.5, 17., 2);
        check_all(0.5, 17., 3);
        check_all(0.5, 17., 1000);
        check_all(0.5, 17., 1001);
    }

    #[test]
    fn other_quantiles() {
        for quantile in vec![0., 0.1, 0.2, 0.75, 0.99, 1.] {
            for num in vec![1, 2, 5, 10, 100, 1000, 1001] {
                check_all(quantile, 17., num);
            }
        }
    }

    fn check_all(quantile: f64, value: f64, num: usize) {
        let it = RandomGenerator::new(quantile, value, num, 17);
        check_one(it, quantile, value, num);

        let it = SequentialGenerator::new(quantile, value, num, SequentialOrder::Ascending);
        check_one(it, quantile, value, num);

        let it = SequentialGenerator::new(quantile, value, num, SequentialOrder::Descending);
        check_one(it, quantile, value, num);
    }

    fn check_one<G: QuantileGenerator>(gen: G, quantile: f64, value: f64, num: usize) {
        // Collect iterator into a vector
        let mut values: Vec<_> = gen.collect();

        // Calculate observed quantile
        values.sort();
        let rank: usize = quantile_to_rank(quantile, num as u64) as usize;
        let actual = values[rank - 1];

        assert_eq!(value, actual.into_inner(), "Sorted values: {:?}", values);
    }
}
