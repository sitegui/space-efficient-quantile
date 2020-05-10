use crate::algorithm::samples_tree::node::{
    Children, InsertResult, Leaf, Node, Nodes, RecordResult, Trunk,
};
use crate::algorithm::samples_tree::Checkpoint;
use std::mem;

/// Represents the root node that can take many forms
#[derive(Debug)]
pub enum Root<S> {
    Leaf(Leaf<S>),
    Trunk(Trunk<S>),
}

impl<S: Ord> Node<S> for Root<S> {
    fn record_sample(
        &mut self,
        sample: S,
        maximal_gap: u64,
        following: &mut Checkpoint<S>,
    ) -> RecordResult<S, Self> {
        match self {
            Root::Leaf(leaf) => {
                let result = leaf.record_sample(sample, maximal_gap, following);
                self.generic_handle_record_result(result)
            }
            Root::Trunk(trunk) => {
                let result = trunk.record_sample(sample, maximal_gap, following);
                self.generic_handle_record_result(result)
            }
        }
    }

    fn insert_max_checkpoint(&mut self, checkpoint: Checkpoint<S>) -> InsertResult<S, Self> {
        match self {
            Root::Leaf(leaf) => {
                let result = leaf.insert_max_checkpoint(checkpoint);
                self.generic_handle_insert_result(result)
            }
            Root::Trunk(trunk) => {
                let result = trunk.insert_max_checkpoint(checkpoint);
                self.generic_handle_insert_result(result)
            }
        }
    }

    fn nodes_to_children(_nodes: Nodes<Self>) -> Children<S> {
        unreachable!("there should be only a single root")
    }

    fn take_from_root(root: &mut Root<S>) -> Self {
        mem::replace(root, Root::Leaf(Leaf::new()))
    }
}

impl<S: Ord> Root<S> {
    fn generic_handle_record_result<N: Node<S>>(
        &mut self,
        result: RecordResult<S, N>,
    ) -> RecordResult<S, Self> {
        match result {
            RecordResult::Inserted(insert_result) => {
                RecordResult::Inserted(self.generic_handle_insert_result(insert_result))
            }
            RecordResult::UpdatedInPlace => RecordResult::UpdatedInPlace,
        }
    }

    fn generic_handle_insert_result<N: Node<S>>(
        &mut self,
        result: InsertResult<S, N>,
    ) -> InsertResult<S, Self> {
        if let InsertResult::Pending(med_checkpoint, right_node) = result {
            // Splitting reached root tree: build new root node
            let left_node = N::take_from_root(self);
            *self = Root::Trunk(Trunk::with_median(
                Box::new(left_node),
                med_checkpoint,
                Box::new(right_node),
            ));
        }

        InsertResult::Done
    }
}

impl<S> Root<S> {
    #[cfg(test)]
    pub fn depth(&self) -> usize {
        match self {
            Root::Leaf(_) => 1,
            Root::Trunk(trunk) => trunk.depth(),
        }
    }
}
