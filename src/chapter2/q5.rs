// 2.5 - Sum Lists
#![allow(dead_code)]

use core::fmt;
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    next: Link,
    value: usize,
}

impl Node {
    fn new(value: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { next: None, value }))
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn prepend(&mut self, value: usize) {
        let new_node = Rc::new(RefCell::new(Node {
            next: self.head.take(),
            value,
        }));

        self.head = Some(new_node);
    }
}

fn sum_lists_recurse(l1: &Link, l2: &Link, carry: usize) -> Link {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let result = Node::new(0);

    let mut value = carry;
    if let Some(node) = l1.as_ref() {
        value += node.borrow().value;
    }

    if let Some(node) = l2.as_ref() {
        value += node.borrow().value;
    }

    result.borrow_mut().value = value % 10;
    let carry_over = if value >= 10 { 1 } else { 0 };

    let more = match (l1, l2) {
        (None, None) => None,
        (None, Some(node2)) => sum_lists_recurse(&None, &node2.borrow().next, carry_over),
        (Some(node1), None) => sum_lists_recurse(&node1.borrow().next, &None, carry_over),
        (Some(node1), Some(node2)) => {
            sum_lists_recurse(&node1.borrow().next, &node2.borrow().next, carry_over)
        }
    };

    result.borrow_mut().next = more;

    Some(result)
}

#[allow(unused_must_use)]
impl Display for LinkedList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        write!(f, "{}", "[");
        while current.is_some() {
            write!(f, "{}", current.clone().unwrap().borrow().value);
            current = current.unwrap().borrow().next.clone();

            if current.is_some() {
                write!(f, "{}", ", ");
            }
        }
        write!(f, "{}", "]")
    }
}

fn get_list(node: Link) -> LinkedList {
    let mut list = LinkedList::new();
    let mut curr = node;

    while let Some(node) = curr {
        list.prepend(node.borrow().value);
        curr = node.borrow().next.clone();
    }

    list
}

#[cfg(test)]
mod tests {
    use crate::chapter2::q5::sum_lists_recurse;

    use super::*;

    #[test]
    fn should_sum_lists() {
        let mut list1 = LinkedList::new();
        list1.prepend(6);
        list1.prepend(1);
        list1.prepend(7);
        let mut list2 = LinkedList::new();
        list2.prepend(2);
        list2.prepend(9);
        list2.prepend(5);

        let result = sum_lists_recurse(&list1.head, &list2.head, 0);
        let actual = get_list(result);
        assert_eq!(format!("{}", actual), "[9, 1, 2]");
    }
}
