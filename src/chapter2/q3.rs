// 1.3 - Delete middle node
#![allow(dead_code)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

struct Node<T> {
    next: Link<T>,
    prev: WeakLink<T>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            value,
        }))
    }
}

struct LinkedList<T> {
    head: Link<T>,
    tail: WeakLink<T>,
    length: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.upgrade().unwrap().borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail)
            }
            None => self.head = Some(new_node.clone()),
        }

        self.tail = Some(Rc::downgrade(&new_node));
        self.length += 1
    }

    fn delete_middle(&mut self) {
        let middle_node = self.length / 2;

        let node_to_delete = self.iter().skip(middle_node).next();

        if let Some(node) = node_to_delete.as_ref() {
            self.remove(node)
        }
    }

    fn remove(&mut self, target: &Rc<RefCell<Node<T>>>) {
        if let Some(next) = target.borrow().next.as_ref() {
            next.borrow_mut().prev = target.borrow().prev.clone();
        } else {
            self.tail = target.borrow().prev.clone();
        }

        if let Some(prev) = target.borrow().prev.as_ref().and_then(|w| w.upgrade()) {
            prev.borrow_mut().next = target.borrow().next.clone();
        } else {
            self.head = target.borrow().next.clone();
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
        }
    }
}

struct Iter<T> {
    next: Link<T>,
}

impl<T> Iterator for Iter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
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
    use super::LinkedList;

    #[test]
    fn should_delete_middle_node() {
        let mut list = LinkedList::new();

        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.append(6);
        list.append(7);
        list.append(8);
        list.append(9);

        list.delete_middle();

        let actual = list
            .iter()
            .map(|value| value.borrow().value)
            .collect::<Vec<i32>>();

        assert!(!actual.contains(&5))
    }
}
