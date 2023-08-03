pub struct ArrayQueue<T> {
    head: usize,
    tail: usize,
    data: Vec<T>,
}

impl<T: Copy> ArrayQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            head: 0,
            tail: 0,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn dequeue(&mut self) -> Option<&T> {
        // 队尾为空
        if self.head == self.tail {
            return None;
        }

        let ret = self.data.get(self.head);
        self.head += 1;

        ret
    }

    pub fn enqueue(&mut self, v: T) -> bool {
        let capacity = self.data.capacity();

        // 队尾已满
        if self.tail == capacity {
            if self.head == 0 {
                return false;
            }

            // 迁移队列中数据
            for i in self.head..self.tail {
                self.data[i - self.head] = self.data[i];
            }

            self.tail -= self.head;
            self.head = 0;
            self.data[self.tail] = v;
        } else {
            self.data.push(v);
        }

        self.tail += 1;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayQueue;

    #[test]
    fn array_queue_it_works() {
        let mut queue = ArrayQueue::new(5);

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);

        assert_eq!(queue.enqueue(6), false);
        assert_eq!(queue.dequeue(), Some(&1));
        assert_eq!(queue.dequeue(), Some(&2));
        assert_eq!(queue.dequeue(), Some(&3));
        assert_eq!(queue.enqueue(6), true);
    }
}
