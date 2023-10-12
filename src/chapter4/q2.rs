// 4.2 - Minimal Tree
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<BinaryNode>>;

struct BinaryNode {
    value: usize,
    left: Option<Link>,
    right: Option<Link>,
}

fn create_minimal_tree(array: &[usize]) -> Option<Link> {
    r_create_minimal_tree(array)
}

fn r_create_minimal_tree(array: &[usize]) -> Option<Link> {
    match array.len() {
        0 => {
            return None;
        }
        1 => {
            return Some(Rc::new(RefCell::new(BinaryNode {
                value: array[0],
                left: None,
                right: None,
            })))
        }
        _ => {
            let l = array.len();
            let (left, right) = array.split_at(l / 2);
            let (r0, r1) = right.split_at(1);
            let root = Some(Rc::new(RefCell::new(BinaryNode {
                value: r0[0],
                left: r_create_minimal_tree(&Vec::from(left)),
                right: r_create_minimal_tree(&Vec::from(r1)),
            })));
            return root;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{create_minimal_tree, Link};

    #[test]
    fn should_build_tree() {
        let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let tree = create_minimal_tree(&sorted_array);
        assert_eq!(sorted_array, in_order_traversal(tree));
    }

    fn in_order_traversal(tree: Option<Link>) -> Vec<usize> {
        let mut path = vec![];
        if let Some(root) = &tree {
            r_in_order_traversal(root, &mut path)
        }

        path
    }

    fn r_in_order_traversal(node: &Link, path: &mut Vec<usize>) {
        if let Some(ref left) = node.borrow().left {
            r_in_order_traversal(left, path)
        }
        path.push(node.borrow().value);
        if let Some(ref right) = node.borrow().right {
            r_in_order_traversal(right, path)
        }
    }
}
