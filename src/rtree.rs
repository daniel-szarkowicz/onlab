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

    pub fn clear(&mut self) {
        self.root = None;
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
                self.aabb = self.aabb.merge(&aabb);
                if let InsertResult::Split(new_node) =
                    nodes.find_best_match(&aabb).insert(aabb, data)
                {
                    nodes.push(new_node);
                    if nodes.len() > NODE_MAX_CHILDREN {
                        let ((aabb1, nodes1), (aabb2, nodes2)) = split(nodes);
                        self.aabb = aabb1;
                        *nodes = nodes1;
                        InsertResult::Split(Self {
                            aabb: aabb2,
                            entry: Entry::Nodes(nodes2),
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
                    let ((aabb1, leaves1), (aabb2, leaves2)) = split(leaves);
                    self.aabb = aabb1;
                    *leaves = leaves1;
                    InsertResult::Split(Self {
                        aabb: aabb2,
                        entry: Entry::Leaves(leaves2),
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

/// Drains the nodes into two vectors using a heuristic to produce smaller AABBs
fn split<T: HasAABB>(nodes: &mut Vec<T>) -> ((AABB, Vec<T>), (AABB, Vec<T>)) {
    let (seed1, seed2) =
        nodes
            .iter()
            .enumerate()
            .flat_map(|(i, n1)| {
                nodes.iter().enumerate().skip(i + 1).map(move |(j, n2)| {
                    (i, j, n1.aabb().merge(n2.aabb()).size())
                })
            })
            .max_by(|(_, _, s1), (_, _, s2)| s1.total_cmp(s2))
            .map(|(i, j, _)| (i, j))
            .unwrap();
    // we need to worry about ordering
    assert!(seed1 < seed2);
    let mut nodes2 = vec![nodes.swap_remove(seed2)];
    let mut nodes1 = vec![nodes.swap_remove(seed1)];
    let mut aabb1 = nodes1[0].aabb().clone();
    let mut aabb2 = nodes2[0].aabb().clone();
    for node in nodes.drain(..) {
        let new_aabb1 = aabb1.merge(node.aabb());
        let new_aabb2 = aabb2.merge(node.aabb());
        let diff1 = new_aabb1.size() - aabb1.size();
        let diff2 = new_aabb2.size() - aabb2.size();
        if diff1 < diff2 {
            nodes1.push(node);
            aabb1 = new_aabb1;
        } else {
            nodes2.push(node);
            aabb2 = new_aabb2;
        }
    }
    ((aabb1, nodes1), (aabb2, nodes2))
}

trait HasAABB {
    fn aabb(&self) -> &AABB;
}

impl<T> HasAABB for Node<T> {
    fn aabb(&self) -> &AABB {
        &self.aabb
    }
}

impl<T> HasAABB for Leaf<T> {
    fn aabb(&self) -> &AABB {
        &self.aabb
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
