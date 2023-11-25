// 4.10 - Check Subtree
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<BinaryNode>>;

struct BinaryNode {
    value: usize,
    left: Option<Link>,
    right: Option<Link>,
}

fn is_subtree(tree1: Option<Link>, tree2: Option<Link>) -> bool {
    r_is_subtree(&tree1, &tree2)
}

fn r_is_subtree(tree1: &Option<Link>, tree2: &Option<Link>) -> bool {
    match (tree1, tree2) {
        (Some(t1), Some(t2)) => {
            if t1.borrow().value == t2.borrow().value && r_compare(tree1, tree2) {
                true
            } else {
                r_is_subtree(&t1.borrow().left, tree2) || r_is_subtree(&t1.borrow().right, tree2)
            }
        }
        (_, None) => true,
        (None, _) => false,
    }
}

fn r_compare(tree1: &Option<Link>, tree2: &Option<Link>) -> bool {
    match (tree1, tree2) {
        (Some(t1), Some(t2)) => {
            t1.borrow().value == t2.borrow().value
                && r_compare(&t1.borrow().left, &t2.borrow().left)
                && r_compare(&t1.borrow().right, &t2.borrow().right)
        }
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::chapter4::q10::{is_subtree, BinaryNode};

    use super::Link;

    #[test]
    fn should_detect_sub_tree() {
        let subtree = sub_tree();
        let leaf3 = Rc::new(RefCell::new(BinaryNode {
            value: 7,
            left: None,
            right: None,
        }));
        let tree1 = Rc::new(RefCell::new(BinaryNode {
            value: 5,
            left: Some(subtree.clone()),
            right: Some(leaf3.clone()),
        }));

        let tree2 = sub_tree();

        // Creating tree2 (subtree)
        assert_eq!(is_subtree(Some(tree1.clone()), Some(tree2)), true);
    }

    fn sub_tree() -> Link {
        let leaf1 = Rc::new(RefCell::new(BinaryNode {
            value: 8,
            left: None,
            right: None,
        }));
        let leaf2 = Rc::new(RefCell::new(BinaryNode {
            value: 9,
            left: None,
            right: None,
        }));
        Rc::new(RefCell::new(BinaryNode {
            value: 6,
            left: Some(leaf1.clone()),
            right: Some(leaf2.clone()),
        }))
    }
}
