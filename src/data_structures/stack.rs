pub struct ArrayStack<T> {
    data: Vec<T>,
}

impl<T> ArrayStack<T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, v: T) {
        self.data.push(v);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}
