// 4.4 - Check Balanced
#![allow(dead_code)]

use super::q2::BinaryNode;
use std::{cell::RefCell, cmp::max, rc::Rc};

type Link = Rc<RefCell<BinaryNode>>;

fn is_balanced(root: Option<Link>) -> bool {
    r_is_balanced(root).1
}

fn r_is_balanced(node: Option<Link>) -> (i32, bool) {
    match node {
        Some(curr) => {
            let left = curr.borrow().left.clone();
            let right = curr.borrow().right.clone();

            let (lh, rh) = (r_is_balanced(left), r_is_balanced(right));

            if !lh.1 || !rh.1 {
                return (0, false);
            }

            if (lh.0 - rh.0).abs() > 1 {
                return (0, false);
            }

            let h = max(lh.0, rh.0);
            return (h + 1, true);
        }
        None => (0, true),
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter4::{q2::create_minimal_tree, q4::is_balanced};

    #[test]
    fn should_be_balanced() {
        let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let tree = create_minimal_tree(&sorted_array);
        assert!(is_balanced(tree));
    }
}
