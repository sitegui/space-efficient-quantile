use std::cmp::Ordering;

/// Represent the samples that were captured as checkpoints
#[derive(Debug, Copy, Clone)]
pub struct Checkpoint<S> {
    /// The captured sample
    sample: S,
    /// The least number of samples between the preceding checkpoint and this one.
    /// Invariant: `min_gap >= 1`
    min_gap: u64,
    /// The greatest number of samples between the preceding checkpoint and this one.
    /// Invariants: `max_gap >= min_gap` and `max_gap <= maximal_gap`
    max_gap: u64,
}

impl<S> Checkpoint<S> {
    /// Return a new checkpoint with the exact knowledge of the sample's rank
    pub fn new_exact(sample: S) -> Self {
        Checkpoint {
            sample,
            min_gap: 1,
            max_gap: 1,
        }
    }

    /// Return a new checkpoint with some approximate knowledge of the sample's rank due to being
    /// inserted before another checkpoint
    pub fn new_preceding(sample: S, following: &Self) -> Self {
        Checkpoint {
            sample,
            min_gap: 1,
            max_gap: following.max_gap,
        }
    }

    /// Return if the checkpoint is a exact sample
    pub fn is_exact(&self) -> bool {
        self.max_gap == 1
    }

    /// Return if this checkpoint can grow to represent one more sample
    pub fn can_grow(&self, maximal_gap: u64) -> bool {
        self.max_gap + 1 <= maximal_gap
    }

    /// Record a new sample in the preceding checkpoint
    pub fn record_before(&mut self) {
        self.min_gap += 1;
        self.max_gap += 1;
    }

    /// Change the capture sample
    pub fn swap_sample(&mut self, new_sample: S) {
        self.sample = new_sample;
    }
}

// Delegate PartialEq, PartialOrd, Eq and Ord to the field `sample`

impl<S: PartialEq> PartialEq for Checkpoint<S> {
    fn eq(&self, other: &Self) -> bool {
        self.sample.eq(&other.sample)
    }
}

impl<S: PartialOrd> PartialOrd for Checkpoint<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sample.partial_cmp(&other.sample)
    }
}

impl<S: Eq> Eq for Checkpoint<S> {}

impl<S: Ord> Ord for Checkpoint<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sample.cmp(&other.sample)
    }
}

impl<S: PartialEq> PartialEq<S> for Checkpoint<S> {
    fn eq(&self, other: &S) -> bool {
        self.sample.eq(other)
    }
}

impl<S: PartialOrd> PartialOrd<S> for Checkpoint<S> {
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        self.sample.partial_cmp(&other)
    }
}
