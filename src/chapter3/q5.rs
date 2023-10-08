// 3.5 - Sort Stack
#![allow(dead_code)]

struct Stack {
    stack: Vec<usize>,
}

impl Stack {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, value: usize) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }

    fn sort(&mut self) {
        let mut r = Vec::new();

        while let Some(curr) = self.stack.pop() {
            while let Some(r_curr) = r.last().copied() {
                if r_curr > curr {
                    self.stack.push(r.pop().unwrap());
                } else {
                    break;
                }
            }

            r.push(curr)
        }

        while let Some(r) = r.pop() {
            self.stack.push(r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn should_sort() {
        let mut stack = Stack::new();

        stack.push(5);
        stack.push(10);
        stack.push(7);
        stack.push(2);
        stack.push(3);
        stack.push(11);

        stack.sort();

        assert_eq!(stack.stack, [11, 10, 7, 5, 3, 2])
    }
}
