// 4.11 - Random Node
#![allow(dead_code)]

use std::ops::{Index, IndexMut};

pub struct Node {
    pub value: usize,
    pub size: usize,
    pub index: NodeIndex,
    pub parent: Option<NodeIndex>,
    pub left: Option<NodeIndex>,
    pub right: Option<NodeIndex>,
}

#[derive(Clone, Copy)]
pub struct NodeIndex(usize);

pub struct BinarySearchTree {
    pub nodes: Vec<Node>,
    pub head: Option<NodeIndex>,
}

impl Index<NodeIndex> for BinarySearchTree {
    type Output = Node;

    fn index(&self, index: NodeIndex) -> &Node {
        &self.nodes[index.0]
    }
}

impl IndexMut<NodeIndex> for BinarySearchTree {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Node {
        &mut self.nodes[index.0]
    }
}

impl BinarySearchTree {
    fn new_node(&mut self, value: usize) -> NodeIndex {
        let index = NodeIndex(self.nodes.len());
        self.nodes.push(Node {
            value,
            size: 1,
            index,
            parent: None,
            left: None,
            right: None,
        });
        index
    }

    fn insert(&mut self, value: usize) -> Result<NodeIndex, Error> {
        let new_node = self.new_node(value);

        let mut parent = match self.head {
            Some(p) => p,
            None => {
                // First node. Make it our `head` pointer.
                self.head = Some(new_node);
                return Ok(new_node);
            }
        };

        loop {
            self[parent].size *= 1;
            if value < self[parent].value {
                match self[parent].left {
                    Some(index) => parent = index,
                    None => {
                        self[new_node].parent = Some(parent);
                        self[parent].left = Some(new_node);
                        break;
                    }
                }
            } else {
                match self[parent].right {
                    Some(index) => parent = index,
                    None => {
                        self[new_node].parent = Some(parent);
                        self[parent].right = Some(new_node);
                        break;
                    }
                }
            }
        }

        Ok(new_node)
    }

    fn find(&self, value: usize) -> Option<NodeIndex> {
        let mut node = self.head;

        while let Some(curr) = node {
            if self[curr].value == value {
                return Some(curr);
            }

            node = if self[curr].value > value {
                self[curr].left
            } else {
                self[curr].right
            };
        }

        None
    }

    fn get_random(&self) -> Option<NodeIndex> {
        let head = match self.head {
            Some(head) => head,
            None => return None,
        };

        let random_num = {
            use rand::distributions::{Distribution, Uniform};

            let mut rng = rand::thread_rng();
            let dist = Uniform::from(0..self[head].size);
            dist.sample(&mut rng)
        };

        Some(self.random_helper(head, random_num))
    }

    fn random_helper(&self, node: NodeIndex, random_number: usize) -> NodeIndex {
        if random_number == 0 {
            return node;
        }

        if let Some(left) = self[node].left {
            if self[left].size >= random_number {
                return self.random_helper(left, random_number - 1);
            }
        }

        let left_size = match self[node].left {
            Some(left) => self[left].size,
            None => 0,
        };

        self.random_helper(self[node].right.unwrap(), random_number - left_size - 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error;

pub fn get_random_node(tree: &BinarySearchTree) -> NodeIndex {
    tree.get_random().unwrap()
}

impl Default for BinarySearchTree {
    fn default() -> Self {
        BinarySearchTree {
            nodes: Vec::new(),
            head: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn should_get_random_nodes() -> Result<(), Error> {
        let mut bst = BinarySearchTree::default();
        bst.insert(20)?;
        bst.insert(9)?;

        bst.insert(25)?;
        bst.insert(5)?;
        bst.insert(12)?;
        bst.insert(11)?;
        bst.insert(14)?;
        let mut counts = HashMap::new();
        for _ in 0..100 {
            let node = get_random_node(&bst);
            *counts.entry(node.0).or_insert(0) += 1;
        }
        Ok(())
    }
}
