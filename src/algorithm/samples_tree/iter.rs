use super::node::SamplesNode;
use super::{Sample, CHILDREN_CAPACITY, NODE_CAPACITY};
use crate::algorithm::samples_tree::{ChildrenArray, SamplesArray};

pub struct IntoIter<T> {
    samples: SamplesArray<T>,
    children: Option<ChildrenArray<T>>,
    next_source: NextSource<T>,
}

enum NextSource<T> {
    Sample,
    Child(Box<IntoIter<T>>),
}

impl<T> IntoIter<T> {
    pub fn new(samples: SamplesArray<T>, mut children: Option<ChildrenArray<T>>) -> Self {
        let next_source = IntoIter::prepare_next_child(&mut children);

        IntoIter {
            samples,
            children,
            next_source,
        }
    }

    fn prepare_next_child(children: &mut Option<ChildrenArray<T>>) -> NextSource<T> {
        todo!()
        // match children {
        //     Some(some_children) => {
        //         let next_source =
        //             NextSource::Child(Box::new(some_children.pop_front().into_iter()));
        //         if some_children.is_empty() {
        //             // No more child nodes
        //             *children = None;
        //         }
        //         next_source
        //     }
        //     None => NextSource::Sample,
        // }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = Sample<T>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
        // loop {
        //     match &mut self.next_source {
        //         NextSource::Child(child_iter) => {
        //             match child_iter.next() {
        //                 next @ Some(_) => return next,
        //                 None => {
        //                     // This child finished
        //                     self.next_source = NextSource::Sample;
        //                 }
        //             }
        //         }
        //         NextSource::Sample => {
        //             if self.samples.is_empty() {
        //                 return None;
        //             }
        //             let next = self.samples.pop_front();
        //             self.next_source = IntoIter::prepare_next_child(&mut self.children);
        //             return Some(next);
        //         }
        //     }
        // }
    }
}

pub struct Iter<'a, T> {
    // The stack point to the next value to return.
    // Once empty, iteration is over
    stack: Vec<(&'a SamplesNode<T>, usize)>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(node: &'a SamplesNode<T>, tree_depth: usize) -> Self {
        let stack = Vec::with_capacity(tree_depth);
        let mut it = Iter { stack };
        it.descend(node);
        it
    }

    fn descend(&mut self, mut node: &'a SamplesNode<T>) {
        loop {
            self.stack.push((node, 0));
            match &node.children {
                None => break,
                Some(children) => node = &children[0],
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a Sample<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (last_node, last_pos) = self.stack.last_mut().unwrap();
            let next = last_node.samples.get(*last_pos);

            match next {
                Some(_) => {
                    *last_pos += 1;
                    if let Some(children) = &last_node.children {
                        // Walk to next sample of the deepest child
                        let child = &children[*last_pos];
                        self.descend(child);
                    }
                    return next;
                }
                None => {
                    // Reached end of the node at the end of the stack
                    self.stack.pop();
                    if self.stack.len() == 0 {
                        return None;
                    }
                }
            }
        }
    }
}
