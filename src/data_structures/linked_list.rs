use std::{marker::PhantomData, ptr::NonNull};

struct Node<T> {
    ele: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    pub fn push_back(&mut self, ele: T) {
        let node = Box::leak(Box::new(Node {
            ele,
            next: None,
            prev: self.tail,
        }))
        .into();

        match self.tail {
            Some(mut tail) => {
                unsafe {
                    tail.as_mut().next = Some(node);
                }
                self.tail = Some(node);
            }
            None => {
                self.head = Some(node);
                self.tail = Some(node);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|tail| {
            match unsafe { tail.as_ref().prev } {
                Some(mut prev) => {
                    unsafe {
                        prev.as_mut().next = None;
                    }
                    self.tail = Some(prev);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }

            let tail = unsafe { Box::from_raw(tail.as_ptr()) };
            tail.ele
        })
    }

    pub fn peek_back(&mut self) -> Option<&T> {
        self.tail.map(|tail| unsafe { &tail.as_ref().ele })
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.peek_back(), Some(&3));
        list.pop_back();
        assert_eq!(list.peek_back(), Some(&2))
    }
}
