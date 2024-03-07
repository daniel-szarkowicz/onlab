#![allow(clippy::new_without_default)]
#![allow(missing_debug_implementations)]

use crate::aabb::AABB;
use std::fmt::Debug;

const NODE_MAX_CHILDREN: usize = 16;

pub struct RTree<T> {
    root: Option<Node<T>>,
}

struct Node<T> {
    aabb: AABB,
    entry: Entry<T>,
}

struct Leaf<T> {
    aabb: AABB,
    data: T,
}

enum Entry<T> {
    Nodes(Vec<Node<T>>),
    Leaves(Vec<Leaf<T>>),
}

impl<T> RTree<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { root: None }
    }

    #[deprecated]
    #[must_use]
    pub fn aabbs(&self) -> Vec<&AABB> {
        let mut collector = vec![];
        if let Some(ref root) = self.root {
            root.aabbs_into(&mut collector);
        }
        collector
    }

    #[must_use]
    pub fn search(&self, aabb: &AABB) -> Vec<&T> {
        let mut collector = vec![];
        if let Some(ref root) = self.root {
            root.search_into(aabb, &mut collector);
        }
        collector
    }

    pub fn insert(&mut self, aabb: AABB, data: T) {
        self.root = Some(if let Some(mut root) = self.root.take() {
            if let InsertResult::Split(new_node) = root.insert(aabb, data) {
                Node {
                    aabb: root.aabb.merge(&new_node.aabb),
                    entry: Entry::Nodes(vec![root, new_node]),
                }
            } else {
                root
            }
        } else {
            Node {
                aabb: aabb.clone(),
                entry: Entry::Leaves(vec![Leaf { aabb, data }]),
            }
        });
    }
}

impl<T> Node<T> {
    fn search_into<'a>(&'a self, aabb: &AABB, collector: &mut Vec<&'a T>) {
        match self.entry {
            Entry::Nodes(ref nodes) => {
                for node in nodes {
                    if node.aabb.overlaps(aabb) {
                        node.search_into(aabb, collector);
                    }
                }
            }
            Entry::Leaves(ref leaves) => {
                for leaf in leaves {
                    if leaf.aabb.overlaps(aabb) {
                        collector.push(&leaf.data);
                    }
                }
            }
        }
    }

    fn insert(&mut self, aabb: AABB, data: T) -> InsertResult<T> {
        match self.entry {
            Entry::Nodes(ref mut nodes) => {
                if let InsertResult::Split(new_node) =
                    nodes.find_best_match(&aabb).insert(aabb, data)
                {
                    self.aabb = self.aabb.merge(&new_node.aabb);
                    nodes.push(new_node);
                    if nodes.len() > NODE_MAX_CHILDREN {
                        // TODO: good split
                        let nodes_half: Vec<Self> =
                            nodes.drain(NODE_MAX_CHILDREN / 2..).collect();
                        self.aabb = nodes
                            .iter()
                            .skip(1)
                            .fold(nodes[0].aabb.clone(), |a, n| {
                                a.merge(&n.aabb)
                            });
                        InsertResult::Split(Self {
                            aabb: nodes_half
                                .iter()
                                .skip(1)
                                .fold(nodes_half[0].aabb.clone(), |a, n| {
                                    a.merge(&n.aabb)
                                }),
                            entry: Entry::Nodes(nodes_half),
                        })
                    } else {
                        InsertResult::NoSplit
                    }
                } else {
                    InsertResult::NoSplit
                }
            }
            Entry::Leaves(ref mut leaves) => {
                self.aabb = self.aabb.merge(&aabb);
                leaves.push(Leaf { aabb, data });
                if leaves.len() > NODE_MAX_CHILDREN {
                    // TODO: good split
                    let leaves_half: Vec<Leaf<T>> =
                        leaves.drain(NODE_MAX_CHILDREN / 2..).collect();
                    self.aabb = leaves
                        .iter()
                        .skip(1)
                        .fold(leaves[0].aabb.clone(), |a, n| a.merge(&n.aabb));
                    InsertResult::Split(Self {
                        aabb: leaves_half
                            .iter()
                            .skip(1)
                            .fold(leaves_half[0].aabb.clone(), |a, n| {
                                a.merge(&n.aabb)
                            }),
                        entry: Entry::Leaves(leaves_half),
                    })
                } else {
                    InsertResult::NoSplit
                }
            }
        }
    }

    #[deprecated]
    fn aabbs_into<'a>(&'a self, collector: &mut Vec<&'a AABB>) {
        collector.push(&self.aabb);
        match self.entry {
            Entry::Nodes(ref nodes) => {
                for n in nodes {
                    n.aabbs_into(collector);
                }
            }
            Entry::Leaves(ref leaves) => {
                for l in leaves {
                    collector.push(&l.aabb);
                }
            }
        }
    }
}

trait FindBestMatch<T> {
    fn find_best_match(&mut self, aabb: &AABB) -> &mut Node<T>;
}

impl<T> FindBestMatch<T> for Vec<Node<T>> {
    fn find_best_match(&mut self, aabb: &AABB) -> &mut Node<T> {
        self.iter_mut()
            .map(|n| (n.aabb.merge(aabb).size() - n.aabb.size(), n))
            .min_by(|(s1, _), (s2, _)| s1.total_cmp(s2))
            .map(|(_, n)| n)
            .unwrap()
    }
}

#[must_use]
enum InsertResult<T> {
    Split(Node<T>),
    NoSplit,
}

impl<T> Debug for RTree<T>
where
    Node<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTree {{ root: {:?} }}", self.root)
    }
}

impl<T> Debug for Node<T>
where
    Entry<T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ aabb: {:?}, entry: {:?} }}",
            self.aabb, self.entry
        )
    }
}

impl<T> Debug for Leaf<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Leaf {{ aabb: {:?}, data: {:?} }}", self.aabb, self.data)
    }
}

impl<T> Debug for Entry<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nodes(v) => write!(f, "Nodes({v:?})"),
            Self::Leaves(v) => write!(f, "Leaf({v:?})"),
        }
    }
}
