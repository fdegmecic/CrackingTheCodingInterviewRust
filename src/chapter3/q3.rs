// 3.3 - Stack of Plates
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    above: Link,
    below: Link,
    value: usize,
}

impl Node {
    fn new(value: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            above: None,
            below: None,
            value,
        }))
    }
}

struct SetOfStacks {
    stacks: Vec<Stack>,
    capacity: usize,
}

impl SetOfStacks {
    fn new(capacity: usize) -> Self {
        SetOfStacks {
            stacks: vec![],
            capacity,
        }
    }

    fn push(&mut self, value: usize) {
        let mut last = self.get_last_stack();

        if last.is_some() && !last.unwrap().is_full() {
        } else {
            let mut stack = Stack::new(self.capacity);
            stack.push(value);
            self.stacks.push(stack)
        }
    }

    fn get_last_stack(&mut self) -> Option<&mut Stack> {
        self.stacks.last_mut()
    }
}

struct Stack {
    capacity: usize,
    top: Link,
    bottom: Link,
    size: usize,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            capacity,
            top: None,
            bottom: None,
            size: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.capacity == self.size
    }

    fn join(&self, above: Link, below: Link) {
        if let Some(below) = below.as_ref() {
            below.borrow_mut().above = above.clone();
        }
        if let Some(above) = above.as_ref() {
            above.borrow_mut().below = below.clone();
        }
    }

    fn push(&mut self, value: usize) -> bool {
        if self.size >= self.capacity {
            return false;
        }

        self.size += 1;
        let new_node = Node::new(value);

        if self.size == 1 {
            self.bottom = Some(new_node.clone());
        }
        self.join(Some(new_node.clone()), self.top.clone());
        self.top = Some(new_node);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::SetOfStacks;

    #[test]
    fn should_add_stacks() {
        let mut stacks = SetOfStacks::new(2);
    }
}
