#![allow(clippy::new_without_default)]
#![allow(missing_debug_implementations)]

use crate::aabb::AABB;
use std::fmt::Debug;

const NODE_MAX_CHINDREN: usize = 16;

pub struct RTree<T> {
    root: Option<RTreeNode<T>>,
}

struct RTreeNode<T> {
    aabb: AABB,
    entry: Entry<T>,
}

enum Entry<T> {
    Nodes(Vec<RTreeNode<T>>),
    Leaf(T),
}

impl<T> RTree<T> {
    #[must_use]
    pub fn new() -> Self {
        todo!()
    }

    #[deprecated]
    pub fn bad_new(aabb: AABB, t: T) -> Self {
        let nodes = vec![RTreeNode {
            aabb: aabb.clone(),
            entry: Entry::Leaf(t),
        }];
        Self {
            root: Some(RTreeNode {
                aabb,
                entry: Entry::Nodes(nodes),
            }),
        }
    }

    #[deprecated]
    pub fn aabbs(&self) -> Vec<&AABB> {
        let mut collector = vec![];
        if let Some(ref root) = self.root {
            root.aabbs_into(&mut collector);
        }
        collector
    }

    pub fn search(&self, aabb: &AABB) -> Vec<&T> {
        let mut collector = vec![];
        if let Some(ref root) = self.root {
            root.search_into(aabb, &mut collector);
        }
        collector
    }

    pub fn insert(&mut self, aabb: AABB, t: T) {
        match self.root {
            Some(ref mut root) => {
                let _result = root.insert(RTreeNode {
                    aabb,
                    entry: Entry::Leaf(t),
                });
            }
            None => {
                self.root = Some(RTreeNode {
                    aabb: aabb.clone(),
                    entry: Entry::Nodes(vec![RTreeNode {
                        aabb,
                        entry: Entry::Leaf(t),
                    }]),
                });
            }
        }
        // if let Some(ref mut root) = self.root {
        //     if let InsertResult::Split(_) = root.insert(RTreeNode {
        //         aabb,
        //         entry: Entry::Leaf(t),
        //     }) {
        //         todo!("handle root splitting")
        //         // let new_root = RTreeNode {
        //         //     aabb: root.aabb.merge(&n.aabb),
        //         //     entry: Entry::Nodes(vec![n, *root]),
        //         // };
        //         // let old_root = std::mem::replace(root, new_root);
        //         // let root_entry =
        //         //     std::mem::replace(&mut root.entry, Entry::Nodes(vec![]));
        //     }
        // } else {
        //     self.root = Some(RTreeNode {
        //         aabb,
        //         entry: Entry::Leaf(t),
        //     });
        // }
    }
}

impl<T> RTreeNode<T> {
    fn search_into<'a>(&'a self, aabb: &AABB, collector: &mut Vec<&'a T>) {
        if self.aabb.overlaps(aabb) {
            match self.entry {
                Entry::Leaf(ref t) => collector.push(t),
                Entry::Nodes(ref nodes) => {
                    for node in nodes {
                        node.search_into(aabb, collector);
                    }
                }
            }
        }
    }

    fn insert(&mut self, node: Self) -> InsertResult<T> {
        match self.entry {
            Entry::Leaf(_) => unreachable!(),
            Entry::Nodes(ref mut nodes) => {
                if let InsertResult::Split(n) = match nodes
                    .iter_mut()
                    .filter(|n| !n.entry.is_leaf())
                    .map(|n| {
                        let size = n.aabb.size();
                        let merged_size = n.aabb.merge(&node.aabb).size();
                        (n, merged_size - size)
                    })
                    .min_by(|(_, sd1), (_, sd2)| sd1.total_cmp(sd2))
                {
                    Some((n, _)) => n.insert(node),
                    None => InsertResult::Split(node),
                } {
                    nodes.push(n);
                    if nodes.len() > NODE_MAX_CHINDREN {
                        todo!("handle splitting");
                    }
                }
                InsertResult::NoSplit
            }
        }
    }

    #[deprecated]
    fn aabbs_into<'a>(&'a self, collector: &mut Vec<&'a AABB>) {
        collector.push(&self.aabb);
        if let Entry::Nodes(ref nodes) = self.entry {
            for n in nodes {
                n.aabbs_into(collector);
            }
        }
    }
}

impl<T> Entry<T> {
    const fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf(_) => true,
            Self::Nodes(_) => false,
        }
    }
}

#[must_use]
enum InsertResult<T> {
    Split(RTreeNode<T>),
    NoSplit,
}

impl<T> Debug for RTree<T>
where
    RTreeNode<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTree {{ root: {:?} }}", self.root)
    }
}

impl<T> Debug for RTreeNode<T>
where
    Entry<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RTreeNode {{ aabb: {:?}, entry: {:?} }}",
            self.aabb, self.entry
        )
    }
}

impl<T> Debug for Entry<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nodes(v) => write!(f, "Nodes({v:?})"),
            Self::Leaf(t) => write!(f, "Leaf({t:?})"),
        }
    }
}
