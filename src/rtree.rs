#![allow(clippy::new_without_default)]
#![allow(missing_debug_implementations)]

use crate::aabb::AABB;

pub struct RTree<T> {
    root: Option<RTreeNode<T>>,
}

enum RTreeNode<T> {
    Nodes(AABB, Vec<RTreeNode<T>>),
    Leaf(AABB, T),
}

impl<T> RTree<T> {
    #[must_use]
    pub fn new() -> Self {
        todo!()
    }

    pub fn search(&self, aabb: &AABB) -> Vec<&T> {
        let mut collector = vec![];
        if let Some(ref root) = self.root {
            root.search_into(aabb, &mut collector);
        };
        collector
    }

    pub fn insert(&mut self, aabb: AABB, t: T) {
        if let Some(ref mut root) = self.root {
            if let InsertResult::Split(node) = root.insert(aabb, t) {
                let new_root = RTreeNode::Nodes(aabb, vec![node, *root]);
                self.root = Some(new_root);
            }
        } else {
            self.root = Some(RTreeNode::Leaf(aabb, t));
        }
    }
}

impl<T> RTreeNode<T> {
    fn search_into<'a>(&'a self, aabb: &AABB, collector: &mut Vec<&'a T>) {
        match self {
            Self::Nodes(n_aabb, nodes) => {
                if n_aabb.overlaps(aabb) {
                    for n in nodes {
                        n.search_into(aabb, collector);
                    }
                }
            }
            Self::Leaf(l_aabb, t) => {
                if l_aabb.overlaps(aabb) {
                    collector.push(t);
                }
            }
        }
    }

    fn insert(&mut self, aabb: AABB, t: T) -> InsertResult<T> {
        match self {
            Self::Nodes(n_aabb, nodes) => {
                *n_aabb = n_aabb.merge(&aabb);
                nodes.push(Self::Leaf(aabb, t));
                InsertResult::NoSplit
                //
            }
            Self::Leaf(_, _) => InsertResult::Split(Self::Leaf(aabb, t)),
        }
    }
}

#[must_use]
enum InsertResult<T> {
    Split(RTreeNode<T>),
    NoSplit,
}
