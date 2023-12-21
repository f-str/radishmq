use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageTopicModel {
    pub name: String,
    pub index: usize,
    pub subscriber: Vec<String>,
}

#[allow(dead_code)]
impl MessageTopicModel {
    pub fn new(name: String, index: usize, subscriber: Vec<String>) -> Self {
        Self {
            name,
            index,
            subscriber,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateMessageTopic {
    pub name: String,
}

#[allow(dead_code)]
impl CreateMessageTopic {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateMessageTopic {
    pub name: String,
}

#[allow(dead_code)]
impl UpdateMessageTopic {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishToMessageTopic<T> {
    pub data: T,
}

#[allow(dead_code)]
impl<T> PublishToMessageTopic<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddPublisherToMessageTopic {
    pub publisher: String,
}

#[allow(dead_code)]
impl AddPublisherToMessageTopic {
    pub fn new(publisher: String) -> Self {
        Self { publisher }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemovePublisherFromMessageTopic {
    pub publisher: String,
}

#[allow(dead_code)]
impl RemovePublisherFromMessageTopic {
    pub fn new(publisher: String) -> Self {
        Self { publisher }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddSubscriberToMessageTopic {
    pub subscriber: String,
}

#[allow(dead_code)]
impl AddSubscriberToMessageTopic {
    pub fn new(subscriber: String) -> Self {
        Self { subscriber }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveSubscriberFromMessageTopic {
    pub subscriber: String,
}

#[allow(dead_code)]
impl RemoveSubscriberFromMessageTopic {
    pub fn new(subscriber: String) -> Self {
        Self { subscriber }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewDataMessageTopic {
    pub new_data: bool,
}

#[allow(dead_code)]
impl NewDataMessageTopic {
    pub fn new(new_data: bool) -> Self {
        Self { new_data }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataMessageTopic<T> {
    pub data: Vec<T>,
}

#[allow(dead_code)]
impl<T> DataMessageTopic<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}
