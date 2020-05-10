use crate::algorithm::samples_tree::checkpoints::{Checkpoints, LeafInsertPos};
use crate::algorithm::samples_tree::node::{
    Children, InsertResult, Node, Nodes, RecordResult, Root,
};
use crate::algorithm::samples_tree::Checkpoint;

/// Represents a leaf node in the B-tree sample structure
#[derive(Debug)]
pub struct Leaf<S> {
    checkpoints: Checkpoints<S>,
}

impl<S: Ord> Node<S> for Leaf<S> {
    fn record_sample(
        &mut self,
        sample: S,
        maximal_gap: u64,
        following: &mut Checkpoint<S>,
    ) -> RecordResult<S, Self> {
        let (pos, following) = self.checkpoints.find_insertion_pos(&sample, following);

        if following.can_grow(maximal_gap) {
            // Drop
            following.record_before();
            RecordResult::UpdatedInPlace
        } else {
            // Insert
            let checkpoint = Checkpoint::new_preceding(sample, following);
            RecordResult::Inserted(self.insert_checkpoint(checkpoint, pos))
        }
    }

    fn insert_max_checkpoint(&mut self, checkpoint: Checkpoint<S>) -> InsertResult<S, Self> {
        if let Some(max) = self.checkpoints.last() {
            debug_assert!(checkpoint > *max);
        }
        self.insert_checkpoint(checkpoint, self.checkpoints.len())
    }

    fn nodes_to_children(nodes: Nodes<Self>) -> Children<S> {
        Children::Leafs(nodes)
    }

    fn take_from_root(root: &mut Root<S>) -> Self {
        match Root::take_from_root(root) {
            Root::Leaf(leaf) => leaf,
            _ => unreachable!("Invalid root node state"),
        }
    }
}

impl<S> Leaf<S> {
    /// Create a new empty leaf node
    pub fn new() -> Self {
        Leaf {
            checkpoints: Checkpoints::new(),
        }
    }

    /// Insert a new checkpoint into this node. If the node is full, it will be split it into
    /// (left, median, right). Self will become left and the other two values will be returned.
    fn insert_checkpoint(
        &mut self,
        checkpoint: Checkpoint<S>,
        pos: usize,
    ) -> InsertResult<S, Self> {
        use InsertResult::*;

        match self.checkpoints.insert_checkpoint(checkpoint, pos) {
            Done => Done,
            Pending(med_checkpoint, right_checkpoints) => Pending(
                med_checkpoint,
                Leaf {
                    checkpoints: right_checkpoints,
                },
            ),
        }
    }
}
