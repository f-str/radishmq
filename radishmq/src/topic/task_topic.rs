use crate::model::task_topic_model::TaskTopicModel;
use log::warn;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct TaskTopic<T: Send + Clone + Debug> {
    pub name: String,
    data: Arc<Mutex<VecDeque<T>>>,
    pub subscriber: Arc<Mutex<Vec<String>>>,
    pub publisher: Arc<Mutex<Vec<String>>>,
}

impl<T: Send + Clone + Debug> TaskTopic<T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            data: Arc::new(Mutex::new(VecDeque::new())),
            subscriber: Arc::new(Mutex::new(Vec::new())),
            publisher: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn publish(&mut self, data_to_add: T) {
        if let Ok(mut data) = self.data.lock() {
            data.push_back(data_to_add);
        } else {
            panic!("TaskTopic::publish() tried to lock a poisoned mutex");
        }
    }

    pub fn publish_multiple(&mut self, data_to_add: &mut Vec<T>) {
        if let Ok(mut data) = self.data.lock() {
            for item in data_to_add.drain(..) {
                data.push_back(item);
            }
        } else {
            panic!("TaskTopic::publish() tried to lock a poisoned mutex");
        }
    }

    pub fn add_publisher(&mut self, identifier: String) {
        if let Ok(mut publisher) = self.publisher.lock() {
            if !publisher.iter().any(|x| *x == identifier) {
                publisher.push(identifier);
            } else {
                warn!("TaskTopic::add_publisher() tried to add a publisher that is already a publisher");
            }
        } else {
            panic!("TaskTopic::add_publisher() tried to lock a poisoned mutex");
        }
    }

    pub fn remove_publisher(&mut self, identifier: String) {
        if let Ok(mut publisher) = self.publisher.lock() {
            if publisher.iter().any(|x| *x == identifier) {
                publisher.retain(|x| *x != identifier);
            } else {
                warn!("TaskTopic::remove_publisher() tried to remove a publisher that is not a publisher");
            }
        } else {
            panic!("TaskTopic::remove_publisher() tried to lock a poisoned mutex");
        }
    }

    pub fn is_publisher(&self, identifier: String) -> bool {
        if let Ok(publisher) = self.publisher.lock() {
            publisher.iter().any(|x| *x == identifier)
        } else {
            panic!("TaskTopic::is_publisher() tried to lock a poisoned mutex");
        }
    }

    pub fn add_subscriber(&mut self, identifier: String) {
        if let Ok(mut subscriber) = self.subscriber.lock() {
            if !subscriber.iter().any(|x| *x == identifier) {
                subscriber.push(identifier);
            } else {
                warn!("TaskTopic::subscribe() tried to subscribe a subscriber that is already subscribed");
            }
        } else {
            panic!("TaskTopic::subscribe() tried to lock a poisoned mutex");
        }
    }

    pub fn remove_subscriber(&mut self, identifier: String) {
        if let Ok(mut subscriber) = self.subscriber.lock() {
            if subscriber.iter().any(|x| *x == identifier) {
                subscriber.retain(|x| *x != identifier);
            } else {
                warn!("TaskTopic::unsubscribe() tried to unsubscribe a subscriber that is not subscribed");
            }
        } else {
            panic!("TaskTopic::unsubscribe() tried to lock a poisoned mutex");
        }
    }

    pub fn has_open_tasks(&self) -> bool {
        if let Ok(data) = self.data.lock() {
            !data.is_empty()
        } else {
            panic!("TaskTopic::has_open_tasks() tried to lock a poisoned mutex");
        }
    }

    pub fn fetch_data(&mut self, identifier: String) -> Option<T> {
        if let Ok(subscriber) = self.subscriber.lock() {
            if !subscriber.iter().any(|x| *x == identifier) {
                panic!("TaskTopic::fetch_data() a subscriber tried to fetch data from a topic it is not subscribed to");
            }
        } else {
            panic!("TaskTopic::fetch_data() tried to lock a poisoned mutex");
        }

        if let Ok(mut data) = self.data.lock() {
            data.pop_front()
        } else {
            panic!("TaskTopic::fetch_data() tried to lock a poisoned mutex");
        }
    }

    pub fn is_subscriber(&self, identifier: String) -> bool {
        if let Ok(subscriber) = self.subscriber.lock() {
            subscriber.iter().any(|x| *x == identifier)
        } else {
            panic!("TaskTopic::is_subscriber() tried to lock a poisoned mutex");
        }
    }

    pub fn to_model(&self) -> TaskTopicModel {
        if let Ok(subscriber) = self.subscriber.lock() {
            TaskTopicModel {
                name: self.name.clone(),
                subscriber: subscriber.clone(),
            }
        } else {
            panic!("TaskTopic::to_model() tried to lock a poisoned mutex");
        }
    }
}
