// 2.1 - Remove Duplicates
#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    next: Link<T>,
    value: T,
}

struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T>
where
    T: Clone + Eq + Hash + Display,
{
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            next: self.head.take(),
            value,
        }));

        self.head = Some(new_node);
    }

    fn remove_duplicates(&mut self) {
        let mut set = HashSet::new();

        let mut prev: Link<T> = None;
        for node in self.iter() {
            let value = node.borrow().value.clone();

            if set.contains(&value) {
                if let Some(previous) = prev.as_ref() {
                    previous.borrow_mut().next = node.borrow().next.clone();
                }
            } else {
                set.insert(value);
                prev = Some(node.clone());
            }
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
        }
    }
}

#[allow(unused_must_use)]
impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        write!(f, "{}", "[");
        while current.is_some() {
            write!(f, "{}", current.clone().unwrap().borrow().value);
            current = current.unwrap().borrow_mut().next.clone();

            if current.is_some() {
                write!(f, "{}", ", ");
            }
        }
        write!(f, "{}", "]")
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
    fn should_remove_duplicates() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(2);
        list.append(4);
        list.append(5);
        list.append(6);
        list.append(6);
        list.remove_duplicates();
        print!("{}", list);
    }
}
