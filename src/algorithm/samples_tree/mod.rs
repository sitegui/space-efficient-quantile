// mod iter;
mod checkpoint;
mod checkpoints;
mod node;
mod tree;

// pub use iter::{IntoIter, Iter};
pub use checkpoint::Checkpoint;
// pub use tree::SamplesTree;

// Max number of elements per node (MUST be even)
const NODE_CAPACITY: usize = 16;

const CHILDREN_CAPACITY: usize = NODE_CAPACITY + 1;
