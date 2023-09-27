// 2.7 - Intersection
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, PartialEq)]
struct Node {
    next: Link,
    value: usize,
}

impl Node {
    fn new(value: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

struct LinkedList {
    head: Link,
    len: usize,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    fn append(&mut self, value: usize) {
        let new_node = Node::new(value);

        if let Some(last) = self.iter().last() {
            last.borrow_mut().next = Some(new_node)
        } else {
            self.head = Some(new_node)
        }
        self.len += 1;
    }

    fn find_intersection(&self, other: &LinkedList) -> Link {
        let last1 = self.iter().last();
        let last2 = other.iter().last();

        if let (Some(l1), Some(l2)) = (last1, last2) {
            if l1.borrow().value != l2.borrow().value {
                return None;
            }
        }

        let (shorter_list, longer_list) = if self.len < other.len {
            (self, other)
        } else {
            (other, self)
        };
        let len_diff = longer_list.len - shorter_list.len;

        longer_list
            .iter()
            .skip(len_diff)
            .zip(shorter_list.iter())
            .find(|(l, r)| l.borrow().value == r.borrow().value)
            .map(|(l, _)| l)
    }

    fn iter(&self) -> Iter {
        Iter {
            next: self.head.clone(),
        }
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
    use super::LinkedList;

    #[test]
    fn should_find_intersetion() {
        let mut list = LinkedList::new();
        list.append(3);
        list.append(1);
        list.append(5);
        list.append(9);
        list.append(7);
        list.append(2);
        list.append(1);

        let mut list1 = LinkedList::new();
        list1.append(4);
        list1.append(6);
        list1.append(7);
        list1.append(2);
        list1.append(1);

        let actual = list.find_intersection(&list1);
        assert_eq!(actual.unwrap().borrow().value, 7)
    }

    #[test]
    fn should_not_find_intersetion() {
        let mut list = LinkedList::new();
        list.append(3);
        list.append(1);
        list.append(5);
        list.append(9);
        list.append(6);
        list.append(2);
        list.append(1);

        let mut list1 = LinkedList::new();
        list1.append(4);
        list1.append(6);
        list1.append(7);
        list1.append(1);
        list1.append(2);

        let actual = list.find_intersection(&list1);
        assert_eq!(actual, None)
    }
}
