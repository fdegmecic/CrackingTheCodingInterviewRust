// 1.2 - Return kth to last
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.head.take() {
            Some(old_head) => {
                new_node.borrow_mut().next = Some(old_head.clone());
            }
            _ => {}
        }

        self.head = Some(new_node);
    }

    fn kth_to_last(&self, k: usize) -> Link<T> {
        let mut p1 = self.iter().next();
        let mut p2 = self.head.clone();

        while let Some(node1) = p1 {
            p1 = node1.borrow().next.clone();
            if let Some(node2) = p2 {
                p2 = node2.borrow().next.clone();
            }
        }

        p1
    }

    fn iter(&self) -> Iter<T> {
        return Iter {
            next: self.head.clone(),
        };
    }
}

struct Iter<T> {
    next: Link<T>,
}

impl<T> Iterator for Iter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(next) => {
                self.next = next.borrow().next.clone();
                Some(next)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn should_return_kth_to_last() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.append(6);
        list.append(7);
        println!("{list:?}");
        let result = list.kth_to_last(3);
        print!("{}", result.unwrap().borrow().value);
    }
}
