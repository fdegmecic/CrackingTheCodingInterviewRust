// 2.6 - Palindrome
#![allow(dead_code)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type Link = Option<Rc<RefCell<Node>>>;
type WeakLink = Option<Weak<RefCell<Node>>>;

struct Node {
    next: Link,
    prev: WeakLink,
    value: usize,
}

impl Node {
    fn new(value: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            value,
        }))
    }
}

struct LinkedList {
    head: Link,
    tail: WeakLink,
    len: usize,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn append(&mut self, value: usize) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.upgrade().unwrap().borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
            }
            None => self.head = Some(new_node.clone()),
        }
        self.len += 1;
        self.tail = Some(Rc::downgrade(&new_node))
    }

    fn is_palidrome(&self) -> bool {
        self.iter()
            .take(self.len / 2 + 1)
            .zip(self.iter().rev().take(self.len / 2 + 1))
            .all(|(l, r)| l.borrow().value == r.borrow().value)
    }

    fn iter(&self) -> Iter {
        Iter {
            next: self.head.clone(),
            prev: self.tail.clone(),
        }
    }
}

struct Iter {
    next: Link,
    prev: WeakLink,
}

impl Iterator for Iter {
    type Item = Rc<RefCell<Node>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|curr| {
            self.next = curr.borrow().next.clone();
            curr
        })
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.prev
            .take()
            .and_then(|prev| prev.upgrade())
            .map(|prev| {
                self.prev = prev.borrow().prev.clone();
                prev
            })
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn should_be_palindrome() {
        let mut list = LinkedList::new();

        list.append(0);
        list.append(1);
        list.append(2);
        list.append(1);
        list.append(0);

        assert!(list.is_palidrome());
    }

    #[test]
    fn should_not_be_palindrome() {
        let mut list = LinkedList::new();

        list.append(0);
        list.append(1);
        list.append(2);
        list.append(1);

        assert!(!list.is_palidrome());
    }
}
