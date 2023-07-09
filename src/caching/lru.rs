/// @see {https://zhuanlan.zhihu.com/p/466409120}
/// https://leetcode.cn/problems/lru-cache/
use std::{borrow::Borrow, collections::HashMap, hash::Hash, marker::PhantomData, ptr::NonNull};

struct Node<K, V> {
    key: K,
    value: V,
    next: Option<NonNull<Node<K, V>>>,
    prev: Option<NonNull<Node<K, V>>>,
}

struct KeyRef<K, V>(NonNull<Node<K, V>>);

impl<K: Eq, V> Borrow<K> for KeyRef<K, V> {
    fn borrow(&self) -> &K {
        unsafe { &self.0.as_ref().key }
    }
}

impl<K: Hash, V> Hash for KeyRef<K, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe {
            self.0.as_ref().key.hash(state);
        }
    }
}

impl<K: Eq, V> PartialEq for KeyRef<K, V> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.as_ref().key.eq(&other.0.as_ref().key) }
    }
}

impl<K: Eq, V> Eq for KeyRef<K, V> {}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            next: None,
            prev: None,
        }
    }
}

pub struct LruCache<K, V> {
    capacity: usize,
    head: Option<NonNull<Node<K, V>>>,
    tail: Option<NonNull<Node<K, V>>>,
    marker: PhantomData<Node<K, V>>,
    map: HashMap<KeyRef<K, V>, NonNull<Node<K, V>>>,
}

impl<K: Eq + PartialEq + Hash, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            head: None,
            tail: None,
            marker: PhantomData,
            map: HashMap::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let node = Box::leak(Box::new(Node::new(key, value))).into();
        let old_node = self.map.remove(&KeyRef(node)).map(|node| {
            self.detach(node);
            node
        });

        if self.map.len() >= self.capacity {
            let tail = self.tail;
            if let Some(node) = tail {
                self.detach(node);
                self.map.remove(&KeyRef(node));
            }
        }

        self.attach(node);
        self.map.insert(KeyRef(node), node);
        old_node.map(|node| unsafe { Box::from_raw(node.as_ptr()).value })
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(node) = self.map.get(key) {
            let node = *node;
            self.detach(node);
            self.attach(node);

            Some(unsafe { &node.as_ref().value })
        } else {
            None
        }
    }

    fn attach(&mut self, mut node: NonNull<Node<K, V>>) {
        match self.head {
            Some(mut head) => {
                unsafe {
                    head.as_mut().prev = Some(node);
                    node.as_mut().next = Some(head);
                    node.as_mut().prev = None;
                }
                self.head = Some(node);
            }
            None => {
                self.tail = Some(node);
                self.head = Some(node);

                unsafe {
                    node.as_mut().prev = None;
                    node.as_mut().next = None;
                }
            }
        }
    }

    fn detach(&mut self, mut node: NonNull<Node<K, V>>) {
        match unsafe { node.as_ref().prev } {
            Some(mut prev) => unsafe {
                prev.as_mut().next = node.as_ref().next;
            },
            None => unsafe {
                self.head = node.as_ref().next;
            },
        }

        match unsafe { node.as_ref().next } {
            Some(mut next) => unsafe {
                next.as_mut().prev = node.as_ref().prev;
            },
            None => unsafe {
                self.tail = node.as_ref().prev;
            },
        }

        unsafe {
            node.as_mut().next = None;
            node.as_mut().prev = None;
        }
    }
}

impl<K, V> Drop for LruCache<K, V> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            self.head = unsafe { node.as_ref().next };
            drop(node.as_ptr())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LruCache;

    #[test]
    fn it_works() {
        let mut lru = LruCache::new(3);

        assert_eq!(lru.put(1, 10), None);
        assert_eq!(lru.put(2, 20), None);
        assert_eq!(lru.put(3, 30), None);
        assert_eq!(lru.get(&1), Some(&10));
        assert_eq!(lru.put(2, 200), Some(20));
        assert_eq!(lru.put(4, 40), None);
        assert_eq!(lru.get(&2), Some(&200));
        assert_eq!(lru.get(&3), None);
    }
}
