use crate::algorithm::samples_tree::checkpoints::Checkpoints;
use crate::algorithm::samples_tree::node::{
    Children, InsertResult, Node, Nodes, RecordResult, Root,
};
use crate::algorithm::samples_tree::{Checkpoint, CHILDREN_CAPACITY};
use arrayvec::ArrayVec;

/// Represents a non-leaf node in the B-tree sample structure
#[derive(Debug)]
pub struct Trunk<S> {
    checkpoints: Checkpoints<S>,
    children: Children<S>,
}

impl<S: Ord> Node<S> for Trunk<S> {
    fn record_sample(
        &mut self,
        sample: S,
        maximal_gap: u64,
        following: &mut Checkpoint<S>,
    ) -> RecordResult<S, Self> {
        match &mut self.children {
            Children::Leafs(leafs) => Trunk::generic_record_sample(
                &mut self.checkpoints,
                leafs,
                sample,
                maximal_gap,
                following,
            ),
            Children::Trunks(trunks) => Trunk::generic_record_sample(
                &mut self.checkpoints,
                trunks,
                sample,
                maximal_gap,
                following,
            ),
        }
    }

    fn insert_max_checkpoint(&mut self, checkpoint: Checkpoint<S>) -> InsertResult<S, Self> {
        match &mut self.children {
            Children::Leafs(leafs) => {
                Trunk::generic_insert_max_checkpoint(&mut self.checkpoints, leafs, checkpoint)
            }
            Children::Trunks(trunks) => {
                Trunk::generic_insert_max_checkpoint(&mut self.checkpoints, trunks, checkpoint)
            }
        }
    }

    fn nodes_to_children(nodes: Nodes<Self>) -> Children<S> {
        Children::Trunks(nodes)
    }

    fn take_from_root(root: &mut Root<S>) -> Self {
        match Root::take_from_root(root) {
            Root::Trunk(trunk) => trunk,
            _ => unreachable!("Invalid root node state"),
        }
    }
}

impl<S: Ord> Trunk<S> {
    fn generic_record_sample<N: Node<S>>(
        checkpoints: &mut Checkpoints<S>,
        nodes: &mut Nodes<N>,
        sample: S,
        maximal_gap: u64,
        following: &mut Checkpoint<S>,
    ) -> RecordResult<S, Self> {
        let (pos, following) = checkpoints.find_insertion_pos(&sample, following);

        use InsertResult::*;
        use RecordResult::*;

        let node = &mut nodes[pos];
        match node.record_sample(sample, maximal_gap, following) {
            // Explicit pass-through to convert `RecordResult<S, N>` into `RecordResult<S, Self>`
            Inserted(Done) => Inserted(Done),
            UpdatedInPlace => UpdatedInPlace,
            Inserted(Pending(med_checkpoint, right_node)) => {
                Inserted(Self::generic_insert_checkpoint(
                    checkpoints,
                    nodes,
                    med_checkpoint,
                    Box::new(right_node),
                    pos,
                ))
            }
        }
    }

    fn generic_insert_max_checkpoint<N: Node<S>>(
        checkpoints: &mut Checkpoints<S>,
        nodes: &mut Nodes<N>,
        checkpoint: Checkpoint<S>,
    ) -> InsertResult<S, Self> {
        use InsertResult::*;

        let last = nodes.last_mut().expect("nodes is not empty");
        match last.insert_max_checkpoint(checkpoint) {
            // Explicit pass-through to convert `InsertResult<S, N>` into `InsertResult<S, Self>`
            Done => Done,
            Pending(med_checkpoint, right_node) => Self::generic_insert_checkpoint(
                checkpoints,
                nodes,
                med_checkpoint,
                Box::new(right_node),
                checkpoints.len(),
            ),
        }
    }
}

impl<S> Trunk<S> {
    pub fn with_median<N: Node<S>>(
        left_node: Box<N>,
        med_checkpoint: Checkpoint<S>,
        right_node: Box<N>,
    ) -> Self {
        let mut nodes: Nodes<N> = ArrayVec::new();
        nodes.push(left_node);
        nodes.push(right_node);

        let mut checkpoints = Checkpoints::new();
        checkpoints.push(med_checkpoint);

        Self::with_children(checkpoints, nodes)
    }

    fn with_children<N: Node<S>>(checkpoints: Checkpoints<S>, nodes: Nodes<N>) -> Self {
        debug_assert_eq!(checkpoints.len() + 1, nodes.len());
        let children = N::nodes_to_children(nodes);
        Trunk {
            checkpoints,
            children,
        }
    }

    /// Insert a new checkpoint into this node. If the node is full, it will be split it into
    /// (left, median, right). Self will become left and the other two values will be returned.
    fn generic_insert_checkpoint<N: Node<S>>(
        checkpoints: &mut Checkpoints<S>,
        nodes: &mut Nodes<N>,
        med_checkpoint: Checkpoint<S>,
        right_node: Box<N>,
        pos: usize,
    ) -> InsertResult<S, Self> {
        use InsertResult::*;

        match checkpoints.insert_checkpoint(med_checkpoint, pos) {
            Done => {
                nodes.insert(pos + 1, right_node);
                Done
            }
            Pending(new_med_checkpoint, right_checkpoints) => {
                let med_pos = nodes.len() / 2;
                let mut right_children: Nodes<N>;
                if pos < med_pos {
                    right_children = nodes.drain(med_pos..).collect();
                    nodes.insert(pos + 1, right_node);
                } else {
                    right_children = nodes.drain(med_pos + 1..).collect();
                    right_children.insert(pos - med_pos, right_node);
                }

                Pending(
                    new_med_checkpoint,
                    Self::with_children(right_checkpoints, right_children),
                )
            }
        }
    }

    #[cfg(test)]
    pub fn depth(&self) -> usize {
        match &self.children {
            Children::Leafs(_) => 2,
            Children::Trunks(trunks) => 1 + trunks[0].depth(),
        }
    }
}
