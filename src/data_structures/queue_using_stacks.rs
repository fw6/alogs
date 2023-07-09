pub struct MyQueue {
    insert_stack: Vec<i32>,
    output_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    pub fn new() -> Self {
        Self {
            insert_stack: Vec::new(),
            output_stack: Vec::new(),
        }
    }

    fn transfer(&mut self) {
        if self.output_stack.is_empty() {
            while let Some(v) = self.insert_stack.pop() {
                self.output_stack.push(v);
            }
        }
    }

    pub fn push(&mut self, x: i32) {
        self.insert_stack.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.transfer();

        self.output_stack.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        self.transfer();
        *self.output_stack.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.insert_stack.is_empty() && self.output_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    fn my_queue_it_works() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());
    }
}
