#set page(height: auto)

```rust
struct RTree {
  root: Option<Node>
}

struct Node {
  aabb: AABB,
  entry: Entry
}

struct Leaf {
  aabb: AABB,
  objid: usize,
}

enum Entry {
  Nodes(Vec<Node>),
  Leaves(Vec<Leaf>),
}
```

2 fontos művelet: Keresés, Beillesztés

= Keresés
Adott AABB-hez megtalálja az összes metsző objektum indexét.
Ez az algoritmus triviális, csak le kell menni mindenhova, ahol
a keresett AABB metszi a csúcspont AABB-jét.

```rust
fn RTree.search(aabb: AABB) -> Vec<usize> {
  if root.is_some() {
    root.search(aabb)
  }
}

fn Node.search(aabb: AABB) -> Vec<usize> {
  match entry {
    Nodes(nodes) => for node in nodes {
      if node.aabb.intersects(aabb) {
        node.search(aabb)
      }
    }
    Leaves(leaves) => for leaf in leaves {
      if leaf.aabb.intersetct(aabb) {
        results.add(leaf.objid)
      }
    }
  }
}
```

= Beillesztés
Ez nehezebb, valahogyan meg kell találni a legjobb helyet a beillesztéshez.
Ha a beillesztéssel több elem lesz a csúcsban, mint a megengedett, akkor
a csúcsot valami alapján vágni kell, a vágást is kezelni kell.

```rust
fn RTree.insert(aabb: AABB, objid: usize) {
  if root.is_none() {
    root = Node(aabb, Leaves(vec![Leaf(aabb, usize)]))
  } else {
    root.insert(aabb, objid);
  }
}

fn Node.insert(aabb: AABB, objid: usize) {
  match entry {
    Nodes(nodes) => nodes.find_best_match(aabb).insert(aabb, objid),
    Leaves(leaves) => leaves.add(Leaf(aabb, objid))
  }
}
```

A fenti kódban a `find_best_match` nincs megvalósítva, mert azt több féle lehet
és nem nagyon számít. A kód még nem kezeli, az csúcsok túlnövését.

```rust
enum InsertResult {
  Split(Node),
  NoSplit,
}

fn RTree.insert(aabb: AABB, objid: usize) {
  if root.is_none() {
    root = Node(aabb, Leaves(vec![Leaf(aabb, usize)]))
  } else {
    if let Split(new_node) = root.insert(aabb, objid) {
      let new_root = Node(
        aabb: new_node.aabb.merge(root.aabb),
        entry: Nodes(vec![root, new_node])
      )
      root = new_root
    }
  }
}

fn Node.insert(aabb: AABB, objid: usize) -> InsertResult {
  match entry {
    Nodes(nodes) => {
      if let Split(new_node) = nodes.find_best_match(aabb).insert(aabb, objid) {
        self.aabb = self.aabb.merge(new_node.aabb)
        nodes.add(new_node);
        if nodes.len() > max {
          // valami szabály alapján split
          return Split(new_node)
        }
      }
    }
    Leaves(leaves) => {
      self.aabb = self.aabb.merge(aabb)
      leaves.add(Leaf(aabb, objid))
      if leaves.len() > max {
        // valami szabály alapján split
        return Split(new_node)
      }
    }
  }
}
```

