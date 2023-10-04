// 3.2 - Stack Min
#![allow(dead_code)]

struct Stack {
    stack: Vec<usize>,
    min: Vec<usize>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            stack: vec![],
            min: vec![],
        }
    }

    fn push(&mut self, value: usize) {
        if value < self.min() {
            self.min.push(value)
        }

        self.stack.push(value)
    }

    fn pop(&mut self) -> Option<usize> {
        let value = self.stack.pop();
        if let Some(value) = value {
            if value == self.min() {
                self.min.pop();
            }
        }
        value
    }

    fn min(&self) -> usize {
        match self.min.last() {
            Some(min) => *min,
            None => usize::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn should_return_min() {
        let mut stack = Stack::new();

        stack.push(5);
        stack.push(4);
        stack.push(3);
        stack.push(2);
        stack.push(1);

        assert_eq!(1, stack.min());

        let value = stack.pop();
        assert_eq!(1, value.unwrap());

        assert_eq!(2, stack.min());
    }
}
