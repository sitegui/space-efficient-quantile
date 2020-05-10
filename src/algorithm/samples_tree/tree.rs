use crate::algorithm::samples_tree::node::{InsertResult, Leaf, Node, RecordResult, Root, Trunk};
use crate::algorithm::samples_tree::Checkpoint;
use std::mem;

/// Represents a tree that records samples into checkpoints
#[derive(Debug)]
pub struct SamplesTree<S> {
    // Store a clone of the minimum sample and the maximum checkpoint separately, because they
    // require special logic
    extremes: Option<(S, Checkpoint<S>)>,
    root: Root<S>,
    // Total number of checkpoints, including the one store at the maximum extreme
    num_checkpoints: usize,
}

impl<S> SamplesTree<S> {
    /// Create a new empty tree
    pub fn new() -> Self {
        SamplesTree {
            extremes: None,
            root: Root::Leaf(Leaf::new()),
            num_checkpoints: 0,
        }
    }

    #[cfg(test)]
    fn depth(&self) -> usize {
        self.root.depth()
    }
}

impl<S: Ord + Clone> SamplesTree<S> {
    /// Record a new sample into this tree, either by a micro-compression or by inserting a new
    /// checkpoint.
    pub fn record_sample(&mut self, sample: S, maximal_gap: u64) {
        match &mut self.extremes {
            None => {
                // First sample
                self.extremes = Some((sample.clone(), Checkpoint::new_exact(sample)));
                self.num_checkpoints += 1;
            }
            Some((_, max_checkpoint)) if *max_checkpoint <= sample => {
                // A new global maximum: check for in-place compression
                if max_checkpoint.can_grow(maximal_gap) {
                    // This is equivalent to insert a new exact checkpoint and then merge the
                    // current max into it
                    max_checkpoint.record_before();
                    max_checkpoint.swap_sample(sample);
                } else {
                    let prev_max_checkpoint =
                        mem::replace(max_checkpoint, Checkpoint::new_exact(sample));
                    self.root.insert_max_checkpoint(prev_max_checkpoint);
                    self.num_checkpoints += 1;
                }
            }
            Some((min_sample, max_checkpoint)) => {
                if *min_sample > sample {
                    // A new global minimum: store it and then apply the general case.
                    // Storing the global minimum is needed to guarantee that small-quantile
                    // queries respect the maximum relative error
                    *min_sample = sample.clone();
                }

                // Generic case
                if let RecordResult::Inserted(_) =
                    self.root.record_sample(sample, maximal_gap, max_checkpoint)
                {
                    self.num_checkpoints += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algorithm::samples_tree::NODE_CAPACITY;

    #[test]
    fn record_asc_depth_1() {
        let mut tree = SamplesTree::new();

        // One full leaf root node
        let n = NODE_CAPACITY;
        for i in 0..n {
            tree.record_sample(i, 1);
        }
        assert_eq!(tree.depth(), 1);
        assert_eq!(tree.num_checkpoints, n);
    }

    #[test]
    fn record_asc_depth_2() {
        let mut tree = SamplesTree::new();

        // One full trunk root node with N half-full leaf nodes and a full leaf node
        let n = NODE_CAPACITY + NODE_CAPACITY * (NODE_CAPACITY / 2) + NODE_CAPACITY;
        for i in 0..n {
            tree.record_sample(i, 1);
        }
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.num_checkpoints, n);
    }

    #[test]
    fn record_asc_depth_3() {
        let mut tree = SamplesTree::new();

        // One full trunk root node with:
        // N x half-full trunk node with (N/2+1) x half-full leaf nodes
        // and one full trunk node with N half-full leaf nodes and a full leaf node
        let n = NODE_CAPACITY
            + NODE_CAPACITY * (NODE_CAPACITY / 2 + (NODE_CAPACITY / 2 + 1) * NODE_CAPACITY / 2)
            + (NODE_CAPACITY + NODE_CAPACITY * (NODE_CAPACITY / 2) + NODE_CAPACITY);
        for i in 0..n {
            tree.record_sample(i, 1);
        }
        assert_eq!(tree.depth(), 3);
        assert_eq!(tree.num_checkpoints, n);
    }
}
