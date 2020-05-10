use crate::algorithm::samples_tree::{Checkpoint, CHILDREN_CAPACITY};

mod leaf;
mod root;
mod trunk;

use arrayvec::ArrayVec;
pub use leaf::*;
pub use root::*;
pub use trunk::*;

pub trait Node<S>: Sized {
    /// Record a new sample into this node, either by a micro-compression or by inserting a new
    /// checkpoint.
    fn record_sample(
        &mut self,
        sample: S,
        // The greatest value for `max_gap`
        maximal_gap: u64,
        // The following checkpoint to this node
        following: &mut Checkpoint<S>,
    ) -> RecordResult<S, Self>;

    /// Insert a checkpoint that is greater than all other checkpoints in this node and its
    /// descendants. It will panic in debug mode if this requirement does not hold true
    fn insert_max_checkpoint(&mut self, checkpoint: Checkpoint<S>) -> InsertResult<S, Self>;

    /// Convert from a generic list of children to the tagged type
    fn nodes_to_children(nodes: Nodes<Self>) -> Children<S>;

    /// Take a not of this type from the root node or panic trying
    fn take_from_root(root: &mut Root<S>) -> Self;
}

/// Represents the children of a non-leaf node in the B-tree sample structure
#[derive(Debug)]
pub enum Children<S> {
    Leafs(Nodes<Leaf<S>>),
    Trunks(Nodes<Trunk<S>>),
}

/// Represents generic children of a non-leaf node in the B-tree sample structure
pub type Nodes<N> = ArrayVec<[Box<N>; CHILDREN_CAPACITY]>;

#[derive(Debug)]
pub enum RecordResult<S, N> {
    UpdatedInPlace,
    Inserted(InsertResult<S, N>),
}

#[derive(Debug)]
pub enum InsertResult<S, N> {
    Done,
    Pending(Checkpoint<S>, N),
}
