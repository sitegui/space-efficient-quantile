use super::{Checkpoint, NODE_CAPACITY};
use crate::algorithm::samples_tree::node::InsertResult;
use arrayvec::ArrayVec;
use std::ops::{Deref, DerefMut};

/// A list of checkpoints using a static-sized array as storage.
///
/// The main advantage over a normal `Vec` is that there is one lesser heap allocation.
#[derive(Debug)]
pub struct Checkpoints<S>(ArrayVec<[Checkpoint<S>; NODE_CAPACITY]>);

#[derive(Debug)]
pub enum LeafInsertPos<'a, S> {
    /// The given sample is larger than all values in the whole tree
    GlobalMax,
    /// Other cases: carry the index and the first checkpoint that is larger than the given sample
    Other(usize, &'a mut Checkpoint<S>),
}

impl<S> Checkpoints<S> {
    /// Create a new empty list of checkpoints
    pub fn new() -> Self {
        Self(ArrayVec::new())
    }

    /// Splits the collection into two at the given index.
    ///
    /// Returns a newly allocated vector containing the elements in the range [at, len). After the
    /// call, the original vector will be left containing the elements [0, at).
    pub fn split_off(&mut self, at: usize) -> Self {
        Self(self.drain(at..).collect())
    }

    /// Insert a new checkpoint into this array. If the array is full, it will be split it into
    /// (left, median, right). Self will become left and the other two values will be returned.
    pub fn insert_checkpoint(
        &mut self,
        checkpoint: Checkpoint<S>,
        pos: usize,
    ) -> InsertResult<S, Self> {
        if !self.is_full() {
            // Simply insert at this node
            self.insert(pos, checkpoint);
            return InsertResult::Done;
        }

        // Node is full: split into two and return median and new node to insert at the parent
        // This part of the code depends on the fact that `CAPACITY` is even to have exactly three
        // cases to handle and generate a perfectly-balanced split
        let med_pos = self.len() / 2;
        let med_checkpoint;
        let mut right_checkpoints;
        if pos < med_pos {
            right_checkpoints = self.split_off(med_pos);
            med_checkpoint = self.pop().expect("left side is non-empty");
            self.insert(pos, checkpoint);
        } else if pos == med_pos {
            right_checkpoints = self.split_off(med_pos);
            med_checkpoint = checkpoint;
        } else {
            right_checkpoints = self.split_off(med_pos + 1);
            med_checkpoint = self.pop().expect("left side is non-empty");
            right_checkpoints.insert(pos - med_pos - 1, checkpoint);
        }

        InsertResult::Pending(med_checkpoint, right_checkpoints)
    }
}

impl<S: Ord> Checkpoints<S> {
    /// Return the insertion position for this sample in a leaf node
    pub fn find_insertion_pos<'a>(
        &'a mut self,
        sample: &S,
        following: &'a mut Checkpoint<S>,
    ) -> (usize, &'a mut Checkpoint<S>) {
        let len = self.len();
        for (i, checkpoint) in self.iter_mut().enumerate() {
            if *checkpoint > *sample {
                return (i, checkpoint);
            }
        }

        (len, following)
    }
}

impl<S> Deref for Checkpoints<S> {
    type Target = ArrayVec<[Checkpoint<S>; NODE_CAPACITY]>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for Checkpoints<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
