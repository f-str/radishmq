use crate::event_queue::event::{
    EventAddPublisherToMessageTopicData, EventAddSubscriberToMessageTopicData,
    EventCreateMessageTopicData, EventDeleteMessageTopicData, EventFetchDataFromMessageTopicData,
    EventPublishToMessageTopicData, EventRemovePublisherFromMessageTopicData,
    EventRemoveSubscriberFromMessageTopicData, TopicEvent,
};
use crate::model::message_topic_model::{
    AddPublisherToMessageTopic, AddSubscriberToMessageTopic, CreateMessageTopic, DataMessageTopic,
    MessageTopicModel, NewDataMessageTopic, PublishToMessageTopic, RemovePublisherFromMessageTopic,
    RemoveSubscriberFromMessageTopic,
};
use crate::topic::message_topic::MessageTopic;
use crate::utils::types::TopicType;
use crate::STATE;
use log::warn;

pub async fn get_all_message_topics() -> Vec<MessageTopicModel> {
    if let Ok(topics) = STATE.message_topics.lock() {
        topics.iter().map(|topic| topic.to_model()).collect()
    } else {
        warn!("TopicService::get_all_message_topics tried to lock a poisoned mutex");
        Vec::new()
    }
}

pub async fn get_message_topic(topic_name: String) -> Option<MessageTopicModel> {
    if let Ok(topics) = STATE.message_topics.lock() {
        topics
            .iter()
            .find(|topic| topic.name == topic_name)
            .map(|topic| topic.to_model())
    } else {
        warn!("TopicService::get_message_topic tried to lock a poisoned mutex");
        None
    }
}

pub async fn create_message_topic(dts: CreateMessageTopic) -> Option<MessageTopicModel> {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        let topic_name = dts.name.clone();
        if topics.iter().any(|topic| topic.name == topic_name) {
            warn!("TopicService::create_message_topic tried to create a message_topic that already exists");
            return None;
        }
        let topic = MessageTopic::new(topic_name.clone());
        topics.push(topic.clone());

        create_create_message_topic_event(dts.clone());

        Some(topic.to_model())
    } else {
        warn!("TopicService::create_message_topic tried to lock a poisoned mutex");
        None
    }
}

fn create_create_message_topic_event(dts: CreateMessageTopic) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::CreateMessageTopic(EventCreateMessageTopicData::new(dts));
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_create_message_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn delete_message_topic(topic_name: String) -> Option<MessageTopicModel> {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        if let Some(index) = topics.iter().position(|topic| topic.name == topic_name) {
            let topic = topics.remove(index);

            create_delete_message_topic_event(topic_name.clone());

            Some(topic.to_model())
        } else {
            warn!("TopicService::delete_message_topic tried to delete a message_topic that does not exist");
            None
        }
    } else {
        warn!("TopicService::delete_message_topic tried to lock a poisoned mutex");
        None
    }
}

fn create_delete_message_topic_event(topic_name: String) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::DeleteMessageTopic(EventDeleteMessageTopicData::new(topic_name));
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_delete_message_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn publish_to_message_topic(
    topic_name: String,
    publisher_identifier: String,
    dts: PublishToMessageTopic<TopicType>,
) {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            // check if publisher_identifier is a publisher of the topic
            if topic.is_publisher(publisher_identifier.clone()) {
                topic.publish(dts.data.clone());

                create_publish_to_message_topic_event(topic_name.clone(), dts.clone());
            } else {
                warn!("TopicService::publish_to_message_topic tried to publish to a message_topic that the publisher is not registered to");
            }
        } else {
            warn!("TopicService::publish_to_message_topic tried to publish to a message_topic that does not exist");
        }
    } else {
        warn!("TopicService::publish_to_message_topic tried to lock a poisoned mutex");
    }
}

fn create_publish_to_message_topic_event(
    topic_name: String,
    dts: PublishToMessageTopic<TopicType>,
) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event =
            TopicEvent::PublishMessageTopic(EventPublishToMessageTopicData::new(topic_name, dts));
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_publish_to_message_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn add_publisher_to_message_topic(topic_name: String, dts: AddPublisherToMessageTopic) {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            topic.add_publisher(dts.publisher.clone());

            create_add_publisher_to_message_topic_event(topic_name.clone(), dts.clone());
        } else {
            warn!("TopicService::add_publisher_to_message_topic tried to add a publisher to a message_topic that does not exist");
        }
    } else {
        warn!("TopicService::add_publisher_to_message_topic tried to lock a poisoned mutex");
    }
}

fn create_add_publisher_to_message_topic_event(
    topic_name: String,
    dts: AddPublisherToMessageTopic,
) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::AddPublisherMessageTopic(EventAddPublisherToMessageTopicData::new(
            topic_name, dts,
        ));
        queue.enqueue(event);
    } else {
        warn!(
            "TopicService::create_add_publisher_to_message_topic_event tried to lock a poisoned mutex"
        );
    }
}

pub async fn remove_publisher_from_message_topic(
    topic_name: String,
    dts: RemovePublisherFromMessageTopic,
) {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            topic.remove_publisher(dts.publisher.clone());

            create_remove_publisher_from_message_topic_event(topic_name.clone(), dts.clone());
        } else {
            warn!("TopicService::remove_publisher_from_message_topic tried to remove a publisher from a message_topic that does not exist");
        }
    } else {
        warn!("TopicService::remove_publisher_from_message_topic tried to lock a poisoned mutex");
    }
}

fn create_remove_publisher_from_message_topic_event(
    topic_name: String,
    dts: RemovePublisherFromMessageTopic,
) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::RemovePublisherMessageTopic(
            EventRemovePublisherFromMessageTopicData::new(topic_name, dts),
        );
        queue.enqueue(event);
    } else {
        warn!(
            "TopicService::create_remove_publisher_from_message_topic_event tried to lock a poisoned mutex"
        );
    }
}

pub async fn add_subscriber_to_message_topic(topic_name: String, dts: AddSubscriberToMessageTopic) {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            topic.add_subscriber(dts.subscriber.clone());

            create_add_subscriber_to_message_topic_event(topic_name.clone(), dts.clone());
        } else {
            warn!("TopicService::add_subscriber_to_message_topic tried to subscribe to a message_topic that does not exist");
        }
    } else {
        warn!("TopicService::add_subscriber_to_message_topic tried to lock a poisoned mutex");
    }
}

fn create_add_subscriber_to_message_topic_event(
    topic_name: String,
    dts: AddSubscriberToMessageTopic,
) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::AddSubscriberMessageTopic(
            EventAddSubscriberToMessageTopicData::new(topic_name, dts),
        );
        queue.enqueue(event);
    } else {
        warn!(
            "TopicService::create_add_subscriber_to_message_topic_event tried to lock a poisoned mutex"
        );
    }
}

pub async fn remove_subscriber_from_message_topic(
    topic_name: String,
    dts: RemoveSubscriberFromMessageTopic,
) {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            topic.remove_subscriber(dts.subscriber.clone());

            create_remove_subscriber_from_message_topic_event(topic_name.clone(), dts.clone());
        } else {
            warn!("TopicService::remove_subscriber_from_message_topic tried to unsubscribe from a message_topic that does not exist");
        }
    } else {
        warn!("TopicService::remove_subscriber_from_message_topic tried to lock a poisoned mutex");
    }
}

fn create_remove_subscriber_from_message_topic_event(
    topic_name: String,
    dts: RemoveSubscriberFromMessageTopic,
) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::RemoveSubscriberMessageTopic(
            EventRemoveSubscriberFromMessageTopicData::new(topic_name, dts),
        );
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_remove_subscriber_from_message_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn is_there_new_data_for_subscriber(
    topic_name: String,
    identifier: String,
) -> Option<NewDataMessageTopic> {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            let new_data = topic.new_data_to_fetch_for_subscriber(identifier);
            Some(NewDataMessageTopic { new_data })
        } else {
            warn!("TopicService::is_there_new_data_for_subscriber tried to get new data for a message_topic that does not exist");
            None
        }
    } else {
        warn!("TopicService::is_there_new_data_for_subscriber tried to lock a poisoned mutex");
        None
    }
}

pub async fn get_new_data_for_subscriber(
    topic_name: String,
    identifier: String,
) -> Option<DataMessageTopic<TopicType>> {
    if let Ok(mut topics) = STATE.message_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            let data = topic
                .get_data_for_subscriber(identifier.clone())
                .map(|data| DataMessageTopic { data });

            create_fetch_data_from_topic_event(
                topic_name.clone(),
                identifier.clone(),
                topic.get_subscriber_index(identifier.clone()),
            );

            data
        } else {
            warn!("TopicService::get_new_data_for_subscriber tried to get new data for a message_topic that does not exist");
            None
        }
    } else {
        warn!("TopicService::get_new_data_for_subscriber tried to lock a poisoned mutex");
        None
    }
}

fn create_fetch_data_from_topic_event(
    topic_name: String,
    subscriber_name: String,
    subscriber_index: usize,
) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::FetchDataMessageTopic(EventFetchDataFromMessageTopicData::new(
            topic_name,
            subscriber_name,
            subscriber_index,
        ));
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_fetch_data_from_topic_event tried to lock a poisoned mutex");
    }
}
