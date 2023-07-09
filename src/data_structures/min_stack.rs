pub struct MinStack {
    min: i32,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        Self {
            min: i32::MAX,
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        if val <= self.min {
            self.min = val;
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        if val == self.min {
            self.min = self.stack.iter().min().unwrap_or(&i32::MAX).to_owned();
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn min_stack_it_works() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}

// pub struct MinStack {
//     min_stack: Vec<i32>,
//     stack: Vec<i32>,
// }

// impl MinStack {
//     pub fn new() -> Self {
//         Self {
//             min_stack: Vec::new(),
//             stack: Vec::new(),
//         }
//     }

//     pub fn push(&mut self, val: i32) {
//         self.stack.push(val);
//         if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
//             self.min_stack.push(val);
//         }
//     }

//     pub fn pop(&mut self) {
//         let val = self.stack.pop().unwrap_or(i32::MIN);

//         let min = self.get_min();
//         if val <= min {
//             self.min_stack.pop();
//         }
//     }

//     pub fn top(&self) -> i32 {
//         *self.stack.last().unwrap()
//     }

//     pub fn get_min(&self) -> i32 {
//         *self.min_stack.last().unwrap_or(&i32::MIN)
//     }
// }
