use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskTopicModel {
    pub name: String,
    pub subscriber: Vec<String>,
}

#[allow(dead_code)]
impl TaskTopicModel {
    pub fn new(name: String, subscriber: Vec<String>) -> Self {
        Self { name, subscriber }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTaskTopic {
    pub name: String,
}

#[allow(dead_code)]
impl CreateTaskTopic {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTaskTopic {
    pub name: String,
}

#[allow(dead_code)]
impl UpdateTaskTopic {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishToTaskTopic<T> {
    pub data: T,
}

#[allow(dead_code)]
impl<T> PublishToTaskTopic<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddPublisherToTaskTopic {
    pub publisher: String,
}

#[allow(dead_code)]
impl AddPublisherToTaskTopic {
    pub fn new(publisher: String) -> Self {
        Self { publisher }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemovePublisherFromTaskTopic {
    pub publisher: String,
}

#[allow(dead_code)]
impl RemovePublisherFromTaskTopic {
    pub fn new(publisher: String) -> Self {
        Self { publisher }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddSubscriberToTaskTopic {
    pub subscriber: String,
}

#[allow(dead_code)]
impl AddSubscriberToTaskTopic {
    pub fn new(subscriber: String) -> Self {
        Self { subscriber }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveSubscriberFromTaskTopic {
    pub subscriber: String,
}

#[allow(dead_code)]
impl RemoveSubscriberFromTaskTopic {
    pub fn new(subscriber: String) -> Self {
        Self { subscriber }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewTasks {
    pub new_tasks: bool,
}

impl NewTasks {
    pub fn new(new_tasks: bool) -> Self {
        Self { new_tasks }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task<T> {
    pub data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Task<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}
