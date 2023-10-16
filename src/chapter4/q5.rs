// 4.5 - Validate BST
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use super::q2::BinaryNode;

type Link = Rc<RefCell<BinaryNode>>;

fn is_valid_bst(root: Option<Link>) -> bool {
    r_is_valid_bst(root, None, None)
}

fn r_is_valid_bst(node: Option<Link>, lower: Option<usize>, upper: Option<usize>) -> bool {
    if let Some(node) = node {
        let value = node.borrow().value;

        if let Some(lower_value) = lower {
            if value <= lower_value {
                return false;
            }
        }

        if let Some(upper_val) = upper {
            if value >= upper_val {
                return false;
            }
        }

        if !r_is_valid_bst(node.borrow().left.clone(), lower, Some(value)) {
            return false;
        }

        if !r_is_valid_bst(node.borrow().right.clone(), Some(value), upper) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::chapter4::{q2::create_minimal_tree, q5::is_valid_bst};

    #[test]
    fn should_be_valid_bst() {
        let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let tree = create_minimal_tree(&sorted_array);
        assert!(is_valid_bst(tree))
    }

    #[test]
    fn should_not_be_valid_bst() {
        let sorted_array = vec![2, 1, 3];
        let tree = create_minimal_tree(&sorted_array);

        assert!(!is_valid_bst(tree));
    }
}
