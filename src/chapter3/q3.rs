// 3.3 - Stack of Plates
#![allow(dead_code)]

use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

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
    pub stacks: Vec<Stack>,
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
        match self.get_last_stack() {
            Some(last) if !last.is_full() => {
                last.push(value);
            }
            _ => {
                let mut stack = Stack::new(self.capacity);
                stack.push(value);
                self.stacks.push(stack);
            }
        }
    }

    fn pop(&mut self) -> Option<usize> {
        let stack = self.get_last_stack()?;
        let value = stack.pop();
        if stack.is_empty() {
            self.stacks.remove(self.stacks.len() - 1);
        }

        value
    }

    fn pop_at(&mut self, index: usize) -> Option<usize> {
        self.left_shift(index)
    }

    fn left_shift(&mut self, index: usize) -> Option<usize> {
        let mut removed_value = None;

        if let Some(stack) = self.stacks.get_mut(index) {
            removed_value = stack.pop();

            if stack.is_empty() {
                self.stacks.remove(index);
            }
        }

        if self.stacks.len() > index + 1 {
            let value = self.left_shift(index + 1);
            if let Some(stack) = self.stacks.get_mut(index) {
                stack.push(value.unwrap());
            }
        }

        removed_value
    }

    fn get_last_stack(&mut self) -> Option<&mut Stack> {
        self.stacks.last_mut()
    }
}

struct Stack {
    capacity: usize,
    top: Link,
    bottom: Link,
    pub size: usize,
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

    fn pop(&mut self) -> Option<usize> {
        let top = self.top.take()?;

        self.top = top.borrow().below.clone();
        self.size -= 1;

        return Some(top.borrow().value);
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Display for SetOfStacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, stack) in self.stacks.iter().enumerate() {
            let mut current = stack.bottom.clone();
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "[")?;
            while let Some(curr) = current {
                write!(f, "{}", curr.borrow().value)?;

                if let Some(next) = curr.borrow().above.clone() {
                    write!(f, ", ")?;
                    current = Some(next);
                } else {
                    break;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::SetOfStacks;

    #[test]
    fn should_add_stacks() {
        let mut stacks = SetOfStacks::new(2);

        stacks.push(1);
        stacks.push(2);
        stacks.push(3);
        stacks.push(4);
        stacks.push(5);
        assert_eq!(format!("{}", stacks), "[1, 2], [3, 4], [5]");

        stacks.pop_at(1);
        assert_eq!(format!("{}", stacks), "[1, 2], [3, 5]");
    }
}
