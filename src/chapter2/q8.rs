// 2.8 - Loop detection
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    next: Link,
    value: usize,
}

impl Node {
    fn new(value: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { next: None, value }))
    }

    fn new_(value: usize, next: Link) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { next, value }))
    }
}

struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, value: usize) {
        let new_node = Node::new(value);

        if let Some(last) = self.iter().last() {
            last.borrow_mut().next = Some(new_node)
        } else {
            self.head = Some(new_node)
        }
    }

    fn add_node(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(last) = self.iter().last() {
            last.borrow_mut().next = Some(node)
        } else {
            self.head = Some(node)
        }
    }

    fn find_loop(&self) -> Link {
        let (tortoise_iter, hare_iter) = (self.iter().skip(1), self.iter().skip(2).step_by(2));

        for (t, h) in tortoise_iter.zip(hare_iter) {
            if Rc::ptr_eq(&t, &h) {
                let fast_iter = self.iter_from_node(h)?;

                for (slow, fast) in self.iter().zip(fast_iter) {
                    if Rc::ptr_eq(&slow, &fast) {
                        return Some(fast);
                    }
                }
            }
        }
        None
    }

    fn iter(&self) -> Iter {
        Iter {
            next: self.head.clone(),
        }
    }

    fn iter_from_node(&self, node: Rc<RefCell<Node>>) -> Option<Iter> {
        for curr in self.iter() {
            if Rc::ptr_eq(&curr, &node) {
                return Some(Iter { next: Some(node) });
            }
        }

        None
    }
}

struct Iter {
    next: Link,
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

#[cfg(test)]
mod tests {
    use super::{LinkedList, Node};

    #[test]
    fn should_find_loop() {
        let mut list = LinkedList::new();
        let node = Node::new(69);
        let circular_linked_nodes = Node::new_(3, Some(Node::new_(1, Some(node.clone()))));
        node.borrow_mut().next = Some(circular_linked_nodes);

        list.append(5);
        list.append(6);
        list.append(7);
        list.append(8);
        list.add_node(node);

        assert_eq!(69, list.find_loop().unwrap().borrow().value);
    }

    #[test]
    fn should_not_find_loop() {
        let mut list = LinkedList::new();

        list.append(5);
        list.append(6);
        list.append(7);
        list.append(8);
        list.append(69);

        assert!(list.find_loop().is_none());
    }
}
