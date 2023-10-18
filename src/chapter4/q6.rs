// 4.6 - Successor
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Rc<RefCell<BinaryNode>>;

#[derive(PartialEq)]
struct BinaryNode {
    value: usize,
    parent: Option<Link>,
    left: Option<Link>,
    right: Option<Link>,
}

fn get_in_order_successor(node: Link) -> Option<Link> {
    if node.borrow().right.is_some() {
        return get_left_most_child(node.borrow().right.clone());
    } else {
        let mut current_node = node;
        let mut parent = current_node.borrow().parent.clone();

        while parent.is_some() && current_node == parent.as_ref().unwrap().borrow().right.clone()? {
            current_node = parent.unwrap();
            parent = current_node.borrow().parent.clone();
        }
        return parent;
    }
}

fn get_left_most_child(node: Option<Link>) -> Option<Link> {
    let iter = LeftMostIter { next: node };

    iter.last()
}

struct LeftMostIter {
    next: Option<Link>,
}

impl Iterator for LeftMostIter {
    type Item = Link;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|curr| {
            self.next = curr.borrow().left.clone();
            curr
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::chapter4::q6::{get_in_order_successor, BinaryNode};

    #[test]
    fn should_get_in_order_successor() {}

    #[test]
    fn test_in_order_successor() {
        // Creating a valid binary tree:
        //       4
        //      / \
        //     2   6
        //    / \
        //   1   3
        let root = Rc::new(RefCell::new(BinaryNode {
            value: 4,
            parent: None,
            left: None,
            right: None,
        }));

        let node1 = Rc::new(RefCell::new(BinaryNode {
            value: 1,
            parent: Some(root.clone()),
            left: None,
            right: None,
        }));

        let node3 = Rc::new(RefCell::new(BinaryNode {
            value: 3,
            parent: Some(root.clone()),
            left: None,
            right: None,
        }));

        let node2 = Rc::new(RefCell::new(BinaryNode {
            value: 2,
            parent: Some(root.clone()),
            left: Some(node1.clone()),
            right: Some(node3.clone()),
        }));

        let node6 = Rc::new(RefCell::new(BinaryNode {
            value: 6,
            parent: Some(root.clone()),
            left: None,
            right: None,
        }));

        root.borrow_mut().left = Some(node2.clone());
        root.borrow_mut().right = Some(node6.clone());

        // Now, let's test the in-order successor of some nodes.
        let successor1 = get_in_order_successor(node1.clone());
        let successor2 = get_in_order_successor(node2.clone());
        let successor3 = get_in_order_successor(node3.clone());
        let successor4 = get_in_order_successor(root.clone());
        let successor6 = get_in_order_successor(node6.clone());

        // Verify the in-order successor for each node.
        assert_eq!(successor1.unwrap().borrow().value, 2);
        assert_eq!(successor2.unwrap().borrow().value, 3);
        assert_eq!(successor3.unwrap().borrow().value, 4);
        assert_eq!(successor4.unwrap().borrow().value, 6);
        assert!(successor6.is_none());
    }
}
