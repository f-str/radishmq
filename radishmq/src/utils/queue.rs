use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Queue<T: Send + Clone> {
    pub queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T: Send + Clone> Queue<T> {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn enqueue(&mut self, data: T) {
        if let Ok(mut queue) = self.queue.lock() {
            queue.push_back(data);
        } else {
            panic!("Queue::enqueue() tried to lock a poisoned mutex")
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if let Ok(mut queue) = self.queue.lock() {
            queue.pop_front()
        } else {
            panic!("Queue::dequeue() tried to lock a poisoned mutex")
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Ok(queue) = self.queue.lock() {
            queue.is_empty()
        } else {
            panic!("Queue::is_empty() tried to lock a poisoned mutex")
        }
    }
}

impl<T: Send + Clone> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}
