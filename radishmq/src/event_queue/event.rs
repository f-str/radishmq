use crate::event_queue::worker::ThreadData;
use crate::model::message_topic_model::{
    AddPublisherToMessageTopic, AddSubscriberToMessageTopic, CreateMessageTopic,
    PublishToMessageTopic, RemovePublisherFromMessageTopic, RemoveSubscriberFromMessageTopic,
};
use crate::model::task_topic_model::{
    AddPublisherToTaskTopic, AddSubscriberToTaskTopic, CreateTaskTopic, PublishToTaskTopic,
    RemovePublisherFromTaskTopic, RemoveSubscriberFromTaskTopic,
};
use crate::repository::{message_topic_repository, task_topic_repository};
use crate::utils::types::TopicType;

#[derive(Clone)]
#[allow(clippy::enum_variant_names)]
pub enum TopicEvent {
    CreateMessageTopic(EventCreateMessageTopicData),
    DeleteMessageTopic(EventDeleteMessageTopicData),
    PublishMessageTopic(EventPublishToMessageTopicData),
    AddPublisherMessageTopic(EventAddPublisherToMessageTopicData),
    RemovePublisherMessageTopic(EventRemovePublisherFromMessageTopicData),
    AddSubscriberMessageTopic(EventAddSubscriberToMessageTopicData),
    RemoveSubscriberMessageTopic(EventRemoveSubscriberFromMessageTopicData),
    FetchDataMessageTopic(EventFetchDataFromMessageTopicData),
    ResetIndexMessageTopic(EventResetIndexOfMessageTopicData),
    CreateTaskTopic(EventCreateTaskTopicData),
    DeleteTaskTopic(EventDeleteTaskTopicData),
    PublishTaskTopic(EventPublishToTaskTopicData),
    AddSubscriberTaskTopic(EventAddSubscriberToTaskTopicData),
    RemoveSubscriberTaskTopic(EventRemoveSubscriberFromTaskTopicData),
    AddPublisherTaskTopic(EventAddPublisherToTaskTopicData),
    RemovePublisherTaskTopic(EventRemovePublisherFromTaskTopicData),
}

impl TopicEvent {
    pub async fn handle(&self, thread_data: ThreadData) {
        match self {
            Self::CreateMessageTopic(data) => data.handle(thread_data).await,
            Self::DeleteMessageTopic(data) => data.handle(thread_data).await,
            Self::PublishMessageTopic(data) => data.handle(thread_data).await,
            Self::AddPublisherMessageTopic(data) => data.handle(thread_data).await,
            Self::RemovePublisherMessageTopic(data) => data.handle(thread_data).await,
            Self::AddSubscriberMessageTopic(data) => data.handle(thread_data).await,
            Self::RemoveSubscriberMessageTopic(data) => data.handle(thread_data).await,
            Self::FetchDataMessageTopic(data) => data.handle(thread_data).await,
            Self::ResetIndexMessageTopic(data) => data.handle(thread_data).await,
            Self::CreateTaskTopic(data) => data.handle(thread_data).await,
            Self::DeleteTaskTopic(data) => data.handle(thread_data).await,
            Self::PublishTaskTopic(data) => data.handle(thread_data).await,
            Self::AddSubscriberTaskTopic(data) => data.handle(thread_data).await,
            Self::RemoveSubscriberTaskTopic(data) => data.handle(thread_data).await,
            Self::AddPublisherTaskTopic(data) => data.handle(thread_data).await,
            Self::RemovePublisherTaskTopic(data) => data.handle(thread_data).await,
        }
    }
}

#[derive(Clone)]
pub struct EventCreateMessageTopicData {
    pub data: CreateMessageTopic,
}

impl EventCreateMessageTopicData {
    pub fn new(data: CreateMessageTopic) -> Self {
        Self { data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::create_message_topic(thread_data, self.data.clone()).await;
    }
}

#[derive(Clone)]
pub struct EventDeleteMessageTopicData {
    pub topic_name: String,
}

impl EventDeleteMessageTopicData {
    pub fn new(topic_name: String) -> Self {
        Self { topic_name }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::delete_message_topic(thread_data, self.topic_name.clone()).await;
    }
}

#[derive(Clone)]
pub struct EventPublishToMessageTopicData {
    pub topic_name: String,
    pub data: PublishToMessageTopic<TopicType>,
}

impl EventPublishToMessageTopicData {
    pub fn new(topic_name: String, data: PublishToMessageTopic<TopicType>) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::publish_to_message_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventAddPublisherToMessageTopicData {
    pub topic_name: String,
    pub data: AddPublisherToMessageTopic,
}

impl EventAddPublisherToMessageTopicData {
    pub fn new(topic_name: String, data: AddPublisherToMessageTopic) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::add_publisher_to_message_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventRemovePublisherFromMessageTopicData {
    pub topic_name: String,
    pub data: RemovePublisherFromMessageTopic,
}

impl EventRemovePublisherFromMessageTopicData {
    pub fn new(topic_name: String, data: RemovePublisherFromMessageTopic) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::remove_publisher_from_message_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventAddSubscriberToMessageTopicData {
    pub topic_name: String,
    pub data: AddSubscriberToMessageTopic,
}

impl EventAddSubscriberToMessageTopicData {
    pub fn new(topic_name: String, data: AddSubscriberToMessageTopic) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::add_subscriber_to_message_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventRemoveSubscriberFromMessageTopicData {
    pub topic_name: String,
    pub data: RemoveSubscriberFromMessageTopic,
}

impl EventRemoveSubscriberFromMessageTopicData {
    pub fn new(topic_name: String, data: RemoveSubscriberFromMessageTopic) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::remove_subscriber_from_message_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventFetchDataFromMessageTopicData {
    pub topic_name: String,
    pub subscriber_name: String,
    pub subscriber_index: usize,
}

impl EventFetchDataFromMessageTopicData {
    pub fn new(topic_name: String, subscriber_name: String, subscriber_index: usize) -> Self {
        Self {
            topic_name,
            subscriber_name,
            subscriber_index,
        }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::fetch_data_from_message_topic(
            thread_data,
            self.topic_name.clone(),
            self.subscriber_name.clone(),
            self.subscriber_index,
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventResetIndexOfMessageTopicData {
    pub topic_name: String,
    pub index_subtrahend: usize,
}

impl EventResetIndexOfMessageTopicData {
    pub fn new(topic_name: String, index_subtrahend: usize) -> Self {
        Self {
            topic_name,
            index_subtrahend,
        }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        message_topic_repository::reset_index_of_message_topic(
            thread_data,
            self.topic_name.clone(),
            self.index_subtrahend,
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventCreateTaskTopicData {
    pub data: CreateTaskTopic,
}

impl EventCreateTaskTopicData {
    pub fn new(data: CreateTaskTopic) -> Self {
        Self { data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        task_topic_repository::create_task_topic(thread_data, self.data.clone()).await;
    }
}

#[derive(Clone)]
pub struct EventDeleteTaskTopicData {
    pub topic_name: String,
}

impl EventDeleteTaskTopicData {
    pub fn new(topic_name: String) -> Self {
        Self { topic_name }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        task_topic_repository::delete_task_topic(thread_data, self.topic_name.clone()).await;
    }
}

#[derive(Clone)]
pub struct EventPublishToTaskTopicData {
    pub topic_name: String,
    pub data: PublishToTaskTopic<TopicType>,
}

impl EventPublishToTaskTopicData {
    pub fn new(topic_name: String, data: PublishToTaskTopic<TopicType>) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        task_topic_repository::publish_to_task_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventAddSubscriberToTaskTopicData {
    pub topic_name: String,
    pub data: AddSubscriberToTaskTopic,
}

impl EventAddSubscriberToTaskTopicData {
    pub fn new(topic_name: String, data: AddSubscriberToTaskTopic) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        task_topic_repository::add_subscriber_to_task_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventRemoveSubscriberFromTaskTopicData {
    pub topic_name: String,
    pub data: RemoveSubscriberFromTaskTopic,
}

impl EventRemoveSubscriberFromTaskTopicData {
    pub fn new(topic_name: String, data: RemoveSubscriberFromTaskTopic) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        task_topic_repository::remove_subscriber_from_task_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventAddPublisherToTaskTopicData {
    pub topic_name: String,
    pub data: AddPublisherToTaskTopic,
}

impl EventAddPublisherToTaskTopicData {
    pub fn new(topic_name: String, data: AddPublisherToTaskTopic) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        task_topic_repository::add_publisher_to_task_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}

#[derive(Clone)]
pub struct EventRemovePublisherFromTaskTopicData {
    pub topic_name: String,
    pub data: RemovePublisherFromTaskTopic,
}

impl EventRemovePublisherFromTaskTopicData {
    pub fn new(topic_name: String, data: RemovePublisherFromTaskTopic) -> Self {
        Self { topic_name, data }
    }

    pub async fn handle(&self, thread_data: ThreadData) {
        task_topic_repository::remove_publisher_from_task_topic(
            thread_data,
            self.topic_name.clone(),
            self.data.clone(),
        )
        .await;
    }
}
