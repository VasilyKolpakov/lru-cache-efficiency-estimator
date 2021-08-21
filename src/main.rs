use crate::BTreeNode::LeafNode;

#[macro_use]
extern crate proptest;

use proptest::prelude::*;
use std::collections::BTreeMap;

fn main() {
    let mut t = BTree::new();
    t.insert(1, 0);
    t.insert(20, 1);
    t.insert(5, 2);
    println!("{:?}", t);
    println!("{:?}", t.get(5));
    println!("{:?}", t.get(50));
}

#[derive(Debug)]
struct BTree {
    node: BTreeNode,
    stats: TreeStats,
}

#[derive(Debug)]
struct TreeStats {
    node_count: u64,
}

#[derive(Debug)]
struct BTreeNodeLink {
    key: u64,
    node: Box<BTreeNode>,
}

#[derive(Debug)]
struct KeyValue {
    key: u64,
    value: u64,
}

#[derive(Debug)]
enum BTreeNode {
    BranchNode { links: Vec<BTreeNodeLink> },
    LeafNode { key_values: Vec<KeyValue> },
}

impl BTree {
    fn new() -> BTree {
        BTree {
            node: LeafNode { key_values: Vec::new() },
            stats: TreeStats { node_count: 1 },
        }
    }

    fn insert(&mut self, key: u64, value: u64) -> () {
        BTree::insert_into_node(&mut self.node, key, value, &mut self.stats)
    }

    fn insert_into_node(node: &mut BTreeNode, key: u64, value: u64, _stats: &mut TreeStats) -> () {
        let current_node = node;
        match current_node {
            BTreeNode::BranchNode { links } => {
                BTree::insert_into_node(links.get_mut(0).unwrap().node.as_mut(), key, value, _stats)
            }
            BTreeNode::LeafNode { key_values } => {
                match key_values.binary_search_by_key(&key, |kv| kv.key) {
                    Ok(i) => key_values[i] = KeyValue { key, value },
                    Err(i) => key_values.insert(i, KeyValue { key, value }),
                }
            }
        }
    }

    fn get(&self, key: u64) -> Option<u64> {
        match &self.node {
            BTreeNode::BranchNode { links } => { unimplemented!("BranchNode is not supported") }
            BTreeNode::LeafNode { key_values } => {
                match key_values.binary_search_by_key(&key, |kv| kv.key) {
                    Ok(i) => Some(key_values.get(i).unwrap().value),
                    Err(_) => None,
                }
            }
        }
    }
}

proptest! {
    #[test]
    fn test_btree(ref v in any::<Vec<(bool, u64)>>()) {
        let mut btree = BTree::new();
        let mut etalon_btree = BTreeMap::new();
        // println!("{:?}", v.len());
        for (is_insert, i) in v {
            let key: u64 = i % 20;
            if *is_insert {
                btree.insert(key, *i + 1);
                etalon_btree.insert(key, *i + 1);
            } else {
                prop_assert!(btree.get(key) == etalon_btree.get(&key).map(|v|*v));
            }
        }
    }
}
