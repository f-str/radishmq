use crate::event_queue::worker::EventQueue;
use crate::topic::message_topic::MessageTopic;
use crate::topic::task_topic::TaskTopic;
use crate::utils::types::TopicType;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State {
    pub message_topics: Arc<Mutex<Vec<MessageTopic<TopicType>>>>,
    pub task_topics: Arc<Mutex<Vec<TaskTopic<TopicType>>>>,
    pub event_queue: Arc<Mutex<EventQueue>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            message_topics: Arc::new(Mutex::new(Vec::new())),
            task_topics: Arc::new(Mutex::new(Vec::new())),
            event_queue: Arc::new(Mutex::new(EventQueue::new())),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
