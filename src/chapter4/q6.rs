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
    }

    get_parent(node)
}

fn get_left_most_child(node: Option<Link>) -> Option<Link> {
    let iter = LeftMostIter { next: node };

    iter.last()
}

fn get_parent(node: Link) -> Option<Link> {


    let node = node.clone();
    let parent = node.borrow().parent.clone();

    if let Some(parent) = parent {
        return if node.borrow().value < parent.borrow().value {
            Some(parent)
        } else {
            get_parent(parent)
        }
    }

    None
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

impl BinaryNode {

    fn new(value: usize) -> Link {
        Rc::new(RefCell::new(BinaryNode {
            value,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::chapter4::q6::{get_in_order_successor, BinaryNode};

    #[test]
    fn should_get_in_order_successor() {
        //       4
        //      / \
        //     2   6
        //    / \
        //   1   3

        let root = BinaryNode::new(4);
        let node1 = BinaryNode::new(1);
        let node2 = BinaryNode::new(2);
        let node3 = BinaryNode::new(3);
        let node6 = BinaryNode::new(6);

        root.borrow_mut().left = Some(node2.clone());
        root.borrow_mut().right = Some(node6.clone());
        node1.borrow_mut().parent = Some(node2.clone());
        node3.borrow_mut().parent = Some(node2.clone());
        node2.borrow_mut().parent = Some(root.clone());
        node2.borrow_mut().left = Some(node1.clone());
        node2.borrow_mut().right = Some(node3.clone());

        let successor1 = get_in_order_successor(node1.clone());
        let successor2 = get_in_order_successor(node2.clone());
        let successor3 = get_in_order_successor(node3.clone());
        let successor4 = get_in_order_successor(root.clone());
        let successor6 = get_in_order_successor(node6.clone());

        assert_eq!(successor1.unwrap().borrow().value, 2);
        assert_eq!(successor2.unwrap().borrow().value, 3);
        assert_eq!(successor3.unwrap().borrow().value, 4);
        assert_eq!(successor4.unwrap().borrow().value, 6);
        assert!(successor6.is_none());
    }
}
