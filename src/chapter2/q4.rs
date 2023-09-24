// 2.4 - Partition
#![allow(dead_code)]

use core::fmt;
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { next: None, value }))
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T>
where
    T: Eq + PartialOrd + Clone,
{
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, value: T) {
        let new_node = Node::new(value);

        if let Some(last) = self.iter().last() {
            last.as_ref().borrow_mut().next = Some(new_node)
        } else {
            self.head = Some(new_node);
        }
    }

    fn partition(&self, partition_value: T) -> Self {
        let mut new_list = LinkedList::new();
        let mut tmp_list = vec![];

        for node in self.iter() {
            if node.borrow().value < partition_value {
                new_list.append(node.borrow().value.clone());
            } else {
                tmp_list.push(node.borrow().value.clone());
            }
        }
        for value in tmp_list.drain(..) {
            new_list.append(value);
        }

        new_list
    }

    fn partition_v2(&mut self, part_element: T) -> Self {
        self.part(|rc_ref| rc_ref.borrow().value < part_element)
            .iter()
            .chain(self.iter())
            .map(|rc| rc.borrow().value.clone())
            .collect()
    }

    fn part<F>(&mut self, mut filter: F) -> Self
    where
        F: FnMut(&Rc<RefCell<Node<T>>>) -> bool,
    {
        self.iter()
            .filter(|rc| filter(rc))
            .map(|rc| rc.borrow().value.clone())
            .collect()
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

impl<A> FromIterator<A> for LinkedList<A>
where
    A: Eq + PartialOrd + Clone,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut list = LinkedList::new();
        for i in iter {
            list.append(i);
        }

        list
    }
}

#[allow(unused_must_use)]
impl<T: Display> Display for LinkedList<T> {
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

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn should_partition() {
        let mut list = LinkedList::new();

        list.append(3);
        list.append(5);
        list.append(8);
        list.append(5);
        list.append(10);
        list.append(2);
        list.append(1);

        let actual = list.partition(5);
        assert_eq!(format!("{}", actual), "[3, 2, 1, 5, 8, 5, 10]");
        let actual_v2 = list.partition_v2(5);
        assert_eq!(format!("{}", actual_v2), "[3, 2, 1, 5, 8, 5, 10]");
    }
}
