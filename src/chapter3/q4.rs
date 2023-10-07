// 3.4 - Queue via Stacks
#![allow(dead_code)]

#[derive(Debug)]
struct MyQueue {
    pub oldest: Vec<usize>,
    pub newest: Vec<usize>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            oldest: Vec::new(),
            newest: Vec::new(),
        }
    }

    fn push(&mut self, value: usize) {
        self.newest.push(value)
    }

    fn pop(&mut self) -> Option<usize> {
        self.shift_stacks(|value| value.pop())
    }

    fn peek(&mut self) -> Option<usize> {
        self.shift_stacks(|value| value.last().copied())
    }

    fn shift_stacks<F>(&mut self, mut function: F) -> Option<usize>
    where
        F: FnMut(&mut Vec<usize>) -> Option<usize>,
    {
        self.shift_newest_to_oldest();
        let value = function(&mut self.oldest);
        value
    }

    fn shift_newest_to_oldest(&mut self) {
        while let Some(value) = self.newest.pop() {
            self.oldest.push(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    fn should_pop_push() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue.push(4);

        assert_eq!(1, queue.peek().unwrap());
        assert_eq!(1, queue.pop().unwrap());
    }
}
