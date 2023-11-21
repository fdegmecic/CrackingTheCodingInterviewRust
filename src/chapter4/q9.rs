// 4.9 - BST Sequences
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<BinaryNode>>;

struct BinaryNode {
    value: usize,
    left: Option<Link>,
    right: Option<Link>,
}

fn bst_sequences(tree: Option<Link>) -> Vec<Vec<usize>> {
    r_bst_sequences(tree)
}

fn r_bst_sequences(tree: Option<Link>) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_get_all_permutations() {}
}
