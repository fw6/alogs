pub struct CircularQueue<T> {
    queue: Vec<T>,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl<T: Copy + Default> CircularQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        Self {
            queue: vec![T::default(); capacity],
            head: 0,
            tail: 0,
            capacity,
        }
    }

    pub fn dequeue(&mut self) -> Option<&T> {
        // 队列为空
        if self.head == self.tail {
            return None;
        }

        let ret = self.queue.get(self.head);
        self.head = (self.head + 1) % self.capacity;

        ret
    }

    pub fn enqueue(&mut self, v: T) -> bool {
        // 队列满了
        if (self.tail + 1) % self.capacity == self.head {
            return false;
        }

        self.queue[self.tail] = v;
        self.tail = (self.tail + 1) % self.capacity;

        true
    }
}

#[cfg(test)]
mod test {
    use super::CircularQueue;

    #[test]
    fn circular_queue_works() {
        let mut queue = CircularQueue::new(4);
        assert!(queue.enqueue(1));
        assert!(queue.enqueue(2));
        assert!(queue.enqueue(3));
        assert!(!queue.enqueue(4));
        assert_eq!(queue.queue, vec![1, 2, 3, 0]);

        assert_eq!(queue.dequeue(), Some(&1));
        assert!(queue.enqueue(4));
        assert_eq!(queue.dequeue(), Some(&2));
        assert_eq!(queue.dequeue(), Some(&3));

        assert_eq!(queue.queue, vec![1, 2, 3, 4]);
        assert_eq!((queue.head, queue.tail), (3, 0));

        assert!(queue.enqueue(5));
        assert!(queue.enqueue(6));
        assert_eq!(queue.queue, vec![5, 6, 3, 4]);
        assert_eq!((queue.head, queue.tail), (3, 2));
    }
}
