//! An iterator over an ordered sequence of floats

use ordered_float::NotNan;
use crate::quantile_to_rank;
use std::iter::{ExactSizeIterator, FusedIterator};
use super::QuantileGenerator;

/// An iterator that will generate sequential values
pub struct SequentialGenerator {
    // `value` could be simply added to `offset`, but we keep them separate to
    // avoid float imprecision and make sure the actual value is returned at the
    // right position
    value: f64,
    position: usize,
    direction: f64,
    offset: f64,
    num: usize,
}

/// The order in which to return the values
pub enum SequentialOrder {
    Ascending,
    Descending,
}

impl SequentialGenerator {
    /// Create a new iterator with the given parameters
    ///
    /// # Example
    /// ```
    /// use fast_quantiles::quantile_generator::*;
    /// use ordered_float::NotNan;
    /// let it = SequentialGenerator::new(0.5, 17., 3, SequentialOrder::Ascending);
    /// let values: Vec<_> = it.collect();
    /// assert_eq!(values, vec![NotNan::from(16.), NotNan::from(17.), NotNan::from(18.)]);
    /// ```
    pub fn new(
        quantile: f64,
        value: f64,
        num: usize,
        order: SequentialOrder,
    ) -> SequentialGenerator {
        assert!(num > 0);
        let rank = quantile_to_rank(quantile, num as u64) as usize;
        let (direction, offset) = match order {
            SequentialOrder::Ascending => (1., -(rank as f64) + 1.),
            _ => (-1., (num - rank) as f64),
        };
        SequentialGenerator {
            value,
            position: 0,
            direction,
            offset,
            num,
        }
    }
}

impl Iterator for SequentialGenerator {
    type Item = NotNan<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        // The terms of the sequence are defined as:
        // v[i] = value + alpha*i + beta
        if self.position == self.num {
            None
        } else {
            let r = self.value + (self.direction * self.position as f64 + self.offset);
            self.position += 1;
            Some(NotNan::from(r))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.num - self.position;
        (size, Some(size))
    }
}

impl FusedIterator for SequentialGenerator {}

impl ExactSizeIterator for SequentialGenerator {}

impl QuantileGenerator for SequentialGenerator {}