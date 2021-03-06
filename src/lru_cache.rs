/*
LRU Cache

Design and implement a data structure for Least Recently Used (LRU) cache. It should support the following operations: get and put.

get(key) - Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.
put(key, value) - Set or insert the value if the key is not already present. When the cache reached its capacity, it should invalidate the least recently used item before inserting a new item.

The cache is initialized with a positive capacity.

Follow up:
Could you do both operations in O(1) time complexity?

Example:
LRUCache cache = new LRUCache( 2 /* capacity */ );
cache.put(1, 1);
cache.put(2, 2);
cache.get(1);       // returns 1
cache.put(3, 3);    // evicts key 2
cache.get(2);       // returns -1 (not found)
cache.put(4, 4);    // evicts key 1
cache.get(1);       // returns -1 (not found)
cache.get(3);       // returns 3
cache.get(4);       // returns 4
 */

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }

    fn evict(&mut self) {
        match (self.left.take(), self.right.take()) {
            (Some(left), Some(right)) => {
                left.borrow_mut().right = Some(Rc::clone(&right));
                right.borrow_mut().left = Some(left);
            }
            (Some(left), None) => {
                left.borrow_mut().right = None;
            }
            (None, Some(right)) => {
                right.borrow_mut().left = None;
            }
            _ => {}
        }
    }
}

struct LinkedListWithMap {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    map: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LinkedListWithMap {
    fn new() -> Self {
        LinkedListWithMap {
            head: None,
            tail: None,
            map: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn add_head(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(Node::new(key, value)));
        if let Some(old_head) = self.head.take() {
            node.borrow_mut().right = Some(Rc::clone(&old_head));
            old_head.borrow_mut().left = Some(Rc::clone(&node));
        } else {
            self.tail = Some(Rc::clone(&node))
        }

        self.head = Some(Rc::clone(&node));
        self.map.insert(key, node);
    }

    fn remove_tail(&mut self) {
        let tail_key = if let Some(old_tail) = self.tail.as_ref() {
            Some(Rc::clone(old_tail).borrow().key)
        } else {
            None
        };

        if let Some(key) = tail_key {
            self.remove(key);
        }
    }

    fn remove(&mut self, key: i32) {
        match self.map.get(&key) {
            Some(node) => {
                if node.borrow().right.is_none() {
                    self.tail = if let Some(left) = node.borrow().left.as_ref() {
                        Some(Rc::clone(left))
                    } else {
                        self.head = None;
                        None
                    }
                }

                if node.borrow().left.is_none() && node.borrow().right.is_some() {
                    self.head = node.borrow_mut().right.take();
                }

                node.borrow_mut().evict();
                self.map.remove(&key);
            }
            None => {}
        }
    }

    fn push(&mut self, key: i32, value: i32) {
        self.remove(key);
        self.add_head(key, value);
    }

    fn get(&self, key: i32) -> Option<i32> {
        if let Some(node) = self.map.get(&key) {
            Some(node.borrow().value)
        } else {
            None
        }
    }
}

struct LRUCache {
    capacity: usize,
    queue: LinkedListWithMap,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            queue: LinkedListWithMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.queue.get(key) {
            self.queue.remove(key);
            self.queue.add_head(key, value);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.queue.push(key, value);
        if self.queue.len() > self.capacity {
            self.queue.remove_tail();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}