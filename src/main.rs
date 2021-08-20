use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

struct BTree {
    node: BTreeNode,
}

struct BTreeNodeLink {
    key: u64,
    node: Box<BTreeNode>,
}

enum BTreeNode {
    BranchNode { links: Vec<BTreeNodeLink> },
    LeafNode { values: Vec<(u64, u64)> },
}

fn find_last_less_or_equal_node(
    search_key: u64,
    links: Vec<Option<Rc<SkipListNode>>>,
    level: usize,
) -> Option<Rc<SkipListNode>> {
    let mut link = links.get(level).unwrap().clone();
    while link.map(|n| n.key <= search_key).unwrap_or(false) {
        link = link.unwrap().links.get(level).unwrap().clone();
    }
    link
}