use crate::model::message_topic_model::MessageTopicModel;
use log::{error, info, warn};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MessageTopic<T: Send + Clone + Debug> {
    pub name: String,
    index: Arc<Mutex<usize>>,
    data: Arc<Mutex<Vec<T>>>,
    subscriber: Arc<Mutex<HashMap<String, usize>>>,
    publisher: Arc<Mutex<Vec<String>>>,
}

impl<T: Send + Clone + Debug> MessageTopic<T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            index: Arc::new(Mutex::new(0)),
            data: Arc::new(Mutex::new(Vec::new())),
            subscriber: Arc::new(Mutex::new(HashMap::new())),
            publisher: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn publish(&mut self, data_to_add: T) {
        if let Ok(mut data) = self.data.lock() {
            data.push(data_to_add);
        } else {
            panic!("MessageTopic::publish() tried to lock a poisoned mutex");
        }
        self.update_index(1);
    }

    #[allow(dead_code)]
    pub fn publish_multiple(&mut self, data_to_add: &mut Vec<T>) {
        if let Ok(mut data) = self.data.lock() {
            data.append(data_to_add);
        } else {
            panic!("MessageTopic::publish() tried to lock a poisoned mutex");
        }
        self.update_index(data_to_add.len());
    }

    fn update_index(&mut self, add_to_index: usize) {
        if add_to_index == usize::MAX {
            panic!("MessageTopic::update_index() tried to add usize::MAX to index");
        }

        if let Ok(mut index) = self.index.lock() {
            match index.checked_add(add_to_index) {
                Some(new_index) => *index = new_index,
                None => {
                    self.reset_index(&mut index);
                    *index += add_to_index;
                }
            }
        } else {
            panic!("MessageTopic::update_index() tried to lock a poisoned mutex");
        }
    }

    fn reset_index(&self, index: &mut usize) {
        if let Ok(mut subscriber) = self.subscriber.lock() {
            let biggest_subscriber_offset = subscriber.values().copied().min().unwrap();

            for (_, offset) in subscriber.iter_mut() {
                *offset -= biggest_subscriber_offset;
            }
            *index -= biggest_subscriber_offset;
        } else {
            panic!("MessageTopic::update_index() tried to lock a poisoned mutex");
        }
    }

    pub fn add_publisher(&mut self, identifier: String) {
        if let Ok(mut publisher) = self.publisher.lock() {
            if !publisher.contains(&identifier) {
                publisher.push(identifier.clone());
                info!(
                    "Added publisher '{}' to message_topic '{}'.",
                    identifier, self.name
                );
            } else {
                warn!("MessageTopic::add_publisher() tried to add an already existing publisher");
            }
        } else {
            error!("MessageTopic::add_publisher() tried to lock a poisoned mutex");
        }
    }

    pub fn is_publisher(&self, identifier: String) -> bool {
        if let Ok(publisher) = self.publisher.lock() {
            publisher.contains(&identifier)
        } else {
            panic!("MessageTopic::is_publisher() tried to lock a poisoned mutex");
        }
    }

    pub fn remove_publisher(&mut self, identifier: String) {
        if let Ok(mut publisher) = self.publisher.lock() {
            if publisher.contains(&identifier) {
                publisher.retain(|x| x != &identifier);
                info!(
                    "Removed publisher '{}' from message_topic '{}'.",
                    identifier, self.name
                );
            } else {
                warn!("MessageTopic::remove_publisher() tried to remove a non-existing publisher");
            }
        } else {
            panic!("MessageTopic::remove_publisher() tried to lock a poisoned mutex");
        }
    }

    pub fn add_subscriber(&mut self, identifier: String) {
        if let Ok(mut subscriber) = self.subscriber.lock() {
            if !subscriber.contains_key(&identifier) {
                subscriber.insert(identifier.clone(), self.get_index());
                info!(
                    "Added subscriber '{}' to message_topic '{}'.",
                    identifier, self.name
                );
            } else {
                warn!("MessageTopic::add_subscriber() tried to add an already existing subscriber");
            }
        } else {
            error!("MessageTopic::add_subscriber() tried to lock a poisoned mutex");
        }
    }

    pub fn remove_subscriber(&mut self, identifier: String) {
        if let Ok(mut subscriber) = self.subscriber.lock() {
            if subscriber.contains_key(&identifier) {
                subscriber.remove(&identifier);
                info!(
                    "Removed subscriber '{}' from message_topic '{}'.",
                    identifier, self.name
                );
            } else {
                warn!(
                    "MessageTopic::remove_subscriber() tried to remove a non-existing subscriber"
                );
            }
        } else {
            panic!("MessageTopic::remove_subscriber() tried to lock a poisoned mutex");
        }
    }

    pub fn get_subscriber_index(&self, identifier: String) -> usize {
        if let Ok(subscriber) = self.subscriber.lock() {
            if subscriber.contains_key(&identifier) {
                subscriber.get(&identifier).cloned().unwrap()
            } else {
                panic!("MessageTopic::get_subscriber_index() tried to get the index of a non-existing subscriber");
            }
        } else {
            panic!("MessageTopic::get_subscriber_index() tried to lock a poisoned mutex");
        }
    }

    pub fn new_data_to_fetch_for_subscriber(&self, identifier: String) -> bool {
        self.get_subscriber_offset(identifier.clone()).unwrap() < self.get_index()
    }

    pub fn get_data_for_subscriber(&mut self, identifier: String) -> Option<Vec<T>> {
        let current_length: usize;
        let ret_val = if let Ok(data) = self.data.lock() {
            current_length = data.len();
            let subscriber_last_fetch = self.get_subscriber_last_fetch(identifier.clone()).unwrap();

            Some(data[subscriber_last_fetch..current_length].to_vec())
        } else {
            panic!("MessageTopic::get_data_for_subscriber() tried to lock a poisoned mutex");
        };
        self.update_subscriber_last_fetch(identifier.clone(), current_length);
        ret_val
    }

    fn get_subscriber_last_fetch(&self, identifier: String) -> Option<usize> {
        if let Ok(subscriber) = self.subscriber.lock() {
            if subscriber.contains_key(&identifier) {
                subscriber.get(&*identifier).cloned()
            } else {
                None
            }
        } else {
            panic!("MessageTopic::get_subscriber_last_fetch() tried to lock a poisoned mutex");
        }
    }

    fn update_subscriber_last_fetch(&mut self, identifier: String, new_last_fetch: usize) {
        if let Ok(mut subscriber) = self.subscriber.lock() {
            if let Entry::Occupied(mut e) = subscriber.entry(identifier) {
                e.insert(new_last_fetch);
            } else {
                warn!("MessageTopic::update_subscriber_last_fetch() tried to update a non-existing subscriber");
            }
        } else {
            panic!("MessageTopic::update_subscriber_last_fetch() tried to lock a poisoned mutex");
        }
    }

    fn get_index(&self) -> usize {
        if let Ok(index) = self.index.lock() {
            *index
        } else {
            panic!("MessageTopic::get_index() tried to lock a poisoned mutex");
        }
    }

    fn get_subscriber_offset(&self, identifier: String) -> Option<usize> {
        if let Ok(subscriber) = self.subscriber.lock() {
            if subscriber.contains_key(&identifier) {
                subscriber.get(&*identifier).cloned()
            } else {
                None
            }
        } else {
            panic!("MessageTopic::get_subscriber_offset() tried to lock a poisoned mutex");
        }
    }

    #[allow(dead_code)]
    pub fn print_subscriber(&self) {
        if let Ok(subscriber) = self.subscriber.lock() {
            println!("Subscriber for message_topic '{}':", self.name);
            for (identifier, offset) in subscriber.iter() {
                println!("{}: {}", identifier, offset);
            }
        } else {
            panic!("MessageTopic::print_subscriber() tried to lock a poisoned mutex");
        }
    }

    #[allow(dead_code)]
    pub fn print_data(&self) {
        if let Ok(data) = self.data.lock() {
            println!("Data for message_topic '{}':", self.name);
            for (index, data) in data.iter().enumerate() {
                println!("{}: {:?}", index, data);
            }
        } else {
            panic!("MessageTopic::print_data() tried to lock a poisoned mutex");
        }
    }

    pub fn to_model(&self) -> MessageTopicModel {
        let subscriber: Vec<String> = self.get_subscriber_names();

        MessageTopicModel {
            name: self.name.clone(),
            index: self.get_index(),
            subscriber,
        }
    }

    fn get_subscriber_names(&self) -> Vec<String> {
        let mut subscriber_names = Vec::new();

        if let Ok(subscriber) = self.subscriber.lock() {
            for (name, _) in subscriber.iter() {
                subscriber_names.push(name.clone());
            }
        } else {
            panic!("MessageTopic::get_subscriber_names() tried to lock a poisoned mutex");
        }

        subscriber_names
    }
}

impl<T: Send + Clone + Debug> Debug for MessageTopic<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageTopic")
            .field("name", &self.name)
            .field("index", &self.get_index())
            .field("data", &self.data)
            .field("subscriber", &self.subscriber)
            .finish()
    }
}
