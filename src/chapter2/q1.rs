// Chapter 2 - Remove Duplicates
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    next: Link<T>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { next: None, value }))
    }
}

struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, value: T) {
        let new_node = Node::new(value);

        if let Some(old_head) = self.head.take() {
            old_head.borrow_mut().next = Some(new_node.clone())
        }

        self.head = Some(new_node);
    }

    fn remove_duplicates(&mut self) {}
}

struct Iter<T>(Rc<RefCell<Node<T>>>);

impl<T> Iterator for Iter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next().take() {
            Some(curr) => {
                self.next = curr.borrow().next.clone();
                Some(curr)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LinkedList, Node};

    #[test]
    fn should_remove_duplicates() {
        let mut list = LinkedList::new().append(1);
    }
}
