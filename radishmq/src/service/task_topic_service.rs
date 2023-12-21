use crate::event_queue::event::{
    EventAddPublisherToTaskTopicData, EventAddSubscriberToTaskTopicData, EventCreateTaskTopicData,
    EventDeleteTaskTopicData, EventPublishToTaskTopicData, EventRemovePublisherFromTaskTopicData,
    EventRemoveSubscriberFromTaskTopicData, TopicEvent,
};
use crate::model::task_topic_model::{
    AddPublisherToTaskTopic, AddSubscriberToTaskTopic, CreateTaskTopic, NewTasks,
    PublishToTaskTopic, RemovePublisherFromTaskTopic, RemoveSubscriberFromTaskTopic,
    TaskTopicModel,
};
use crate::topic::task_topic::TaskTopic;
use crate::utils::types::TopicType;
use crate::STATE;
use log::warn;

pub async fn get_all_task_topics() -> Vec<TaskTopicModel> {
    if let Ok(topics) = STATE.task_topics.lock() {
        topics.iter().map(|topic| topic.to_model()).collect()
    } else {
        warn!("TopicService::get_all_task_topics tried to lock a poisoned mutex");
        Vec::new()
    }
}

pub async fn get_task_topic(topic_name: String) -> Option<TaskTopicModel> {
    if let Ok(topics) = STATE.task_topics.lock() {
        topics
            .iter()
            .find(|topic| topic.name == topic_name)
            .map(|topic| topic.to_model())
    } else {
        warn!("TopicService::get_task_topic tried to lock a poisoned mutex");
        None
    }
}

pub async fn create_task_topic(dts: CreateTaskTopic) -> Option<TaskTopicModel> {
    if let Ok(mut topics) = STATE.task_topics.lock() {
        let topic_name = dts.name.clone();
        if topics.iter().any(|topic| topic.name == topic_name) {
            warn!(
                "TopicService::create_task_topic tried to create a task_topic that already exists"
            );
            return None;
        }
        let topic = TaskTopic::new(topic_name.clone());
        topics.push(topic.clone());

        create_create_task_topic_event(dts.clone());

        Some(topic.to_model())
    } else {
        warn!("TopicService::create_task_topic tried to lock a poisoned mutex");
        None
    }
}

fn create_create_task_topic_event(dts: CreateTaskTopic) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::CreateTaskTopic(EventCreateTaskTopicData::new(dts));
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_create_task_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn delete_task_topic(topic_name: String) -> Option<TaskTopicModel> {
    if let Ok(mut topics) = STATE.task_topics.lock() {
        if let Some(index) = topics.iter().position(|topic| topic.name == topic_name) {
            let topic = topics.remove(index);

            create_delete_task_topic_event(topic_name.clone());

            Some(topic.to_model())
        } else {
            warn!(
                "TopicService::delete_task_topic tried to delete a task_topic that does not exist"
            );
            None
        }
    } else {
        warn!("TopicService::delete_task_topic tried to lock a poisoned mutex");
        None
    }
}

fn create_delete_task_topic_event(topic_name: String) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::DeleteTaskTopic(EventDeleteTaskTopicData::new(topic_name));
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_delete_task_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn add_publisher_to_task_topic(topic_name: String, dts: AddPublisherToTaskTopic) {
    if let Ok(mut topics) = STATE.task_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            if !topic.is_publisher(dts.publisher.clone()) {
                topic.add_publisher(dts.publisher.clone());

                create_add_publisher_to_task_topic_event(topic_name.clone(), dts.clone())
            } else {
                warn!("TopicService::add_publisher_to_task_topic tried to add a publisher to a task_topic that the publisher is already a publisher of");
            }
        } else {
            warn!("TopicService::add_publisher_to_task_topic tried to add a publisher to a task_topic that does not exist");
        }
    } else {
        warn!("TopicService::add_publisher_to_task_topic tried to lock a poisoned mutex");
    }
}

fn create_add_publisher_to_task_topic_event(topic_name: String, dts: AddPublisherToTaskTopic) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::AddPublisherTaskTopic(EventAddPublisherToTaskTopicData::new(
            topic_name, dts,
        ));
        queue.enqueue(event);
    } else {
        warn!(
            "TopicService::create_add_publisher_to_task_topic_event tried to lock a poisoned mutex"
        );
    }
}

pub async fn remove_publisher_from_task_topic(
    topic_name: String,
    dts: RemovePublisherFromTaskTopic,
) {
    if let Ok(mut topics) = STATE.task_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            if topic.is_publisher(dts.publisher.clone()) {
                topic.remove_publisher(dts.publisher.clone());

                create_remove_publisher_from_task_topic_event(topic_name.clone(), dts.clone())
            } else {
                warn!("TopicService::remove_publisher_from_task_topic tried to remove a publisher from a task_topic that the publisher is not a publisher of");
            }
        } else {
            warn!("TopicService::remove_publisher_from_task_topic tried to remove a publisher from a task_topic that does not exist");
        }
    } else {
        warn!("TopicService::remove_publisher_from_task_topic tried to lock a poisoned mutex");
    }
}

fn create_remove_publisher_from_task_topic_event(
    topic_name: String,
    dts: RemovePublisherFromTaskTopic,
) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::RemovePublisherTaskTopic(
            EventRemovePublisherFromTaskTopicData::new(topic_name, dts),
        );
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_remove_publisher_from_task_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn publish_to_task_topic(
    topic_name: String,
    publisher_identifier: String,
    dts: PublishToTaskTopic<TopicType>,
) {
    if let Ok(mut topics) = STATE.task_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            if topic.is_publisher(publisher_identifier) {
                topic.publish(dts.data.clone());

                create_publish_to_task_topic_event(topic_name.clone(), dts.clone())
            } else {
                warn!("TopicService::publish_to_task_topic tried to publish to a task_topic that the publisher is not a publisher of");
            }
        } else {
            warn!("TopicService::publish_to_task_topic tried to publish to a task_topic that does not exist");
        }
    } else {
        warn!("TopicService::publish_to_task_topic tried to lock a poisoned mutex");
    }
}

fn create_publish_to_task_topic_event(topic_name: String, dts: PublishToTaskTopic<TopicType>) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::PublishTaskTopic(EventPublishToTaskTopicData::new(topic_name, dts));
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_publish_to_task_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn add_subscriber_to_task_topic(topic_name: String, dts: AddSubscriberToTaskTopic) {
    if let Ok(mut topics) = STATE.task_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            if !topic.is_subscriber(dts.subscriber.clone()) {
                topic.add_subscriber(dts.subscriber.clone());
                create_subscribe_to_task_topic_event(topic_name.clone(), dts.clone())
            } else {
                warn!("TopicService::subscribe_to_task_topic tried to subscribe to a task_topic that the subscriber is already subscribed to");
            }
        } else {
            warn!("TopicService::subscribe_to_task_topic tried to subscribe to a task_topic that does not exist");
        }
    } else {
        warn!("TopicService::subscribe_to_task_topic tried to lock a poisoned mutex");
    }
}

fn create_subscribe_to_task_topic_event(topic_name: String, dts: AddSubscriberToTaskTopic) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::AddSubscriberTaskTopic(EventAddSubscriberToTaskTopicData::new(
            topic_name, dts,
        ));
        queue.enqueue(event);
    } else {
        warn!("TopicService::create_subscribe_to_task_topic_event tried to lock a poisoned mutex");
    }
}

pub async fn remove_subscriber_from_task_topic(
    topic_name: String,
    dts: RemoveSubscriberFromTaskTopic,
) {
    if let Ok(mut topics) = STATE.task_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            if topic.is_subscriber(dts.subscriber.clone()) {
                topic.remove_subscriber(dts.subscriber.clone());

                create_unsubscribe_from_task_topic_event(topic_name.clone(), dts.clone())
            } else {
                warn!("TopicService::unsubscribe_from_task_topic tried to unsubscribe from a task_topic that the subscriber is not subscribed to");
            }
        } else {
            warn!("TopicService::unsubscribe_from_task_topic tried to unsubscribe from a task_topic that does not exist");
        }
    } else {
        warn!("TopicService::unsubscribe_from_task_topic tried to lock a poisoned mutex");
    }
}

fn create_unsubscribe_from_task_topic_event(
    topic_name: String,
    dts: RemoveSubscriberFromTaskTopic,
) {
    if let Ok(mut queue) = STATE.event_queue.lock() {
        let event = TopicEvent::RemoveSubscriberTaskTopic(
            EventRemoveSubscriberFromTaskTopicData::new(topic_name, dts),
        );
        queue.enqueue(event);
    } else {
        warn!(
            "TopicService::create_unsubscribe_from_task_topic_event tried to lock a poisoned mutex"
        );
    }
}

pub async fn is_there_a_task_for_subscriber(topic_name: String, subscriber: String) -> NewTasks {
    if let Ok(topics) = STATE.task_topics.lock() {
        if let Some(topic) = topics.iter().find(|topic| topic.name == topic_name) {
            if topic.is_subscriber(subscriber.clone()) {
                NewTasks::new(topic.has_open_tasks())
            } else {
                warn!("TopicService::is_there_a_task_for_subscriber tried to check a task_topic that the subscriber is not subscribed to");
                NewTasks::new(false)
            }
        } else {
            warn!("TopicService::is_there_a_task_for_subscriber tried to check a task_topic that does not exist");
            NewTasks::new(false)
        }
    } else {
        warn!("TopicService::is_there_a_task_for_subscriber tried to lock a poisoned mutex");
        NewTasks::new(false)
    }
}

pub async fn get_new_task_for_subscriber(
    topic_name: String,
    subscriber: String,
) -> Option<TopicType> {
    if let Ok(mut topics) = STATE.task_topics.lock() {
        if let Some(topic) = topics.iter_mut().find(|topic| topic.name == topic_name) {
            if topic.is_subscriber(subscriber.clone()) {
                topic.fetch_data(subscriber)
            } else {
                warn!("TopicService::get_new_task_for_subscriber tried to get a task from a task_topic that the subscriber is not subscribed to");
                None
            }
        } else {
            warn!("TopicService::get_new_task_for_subscriber tried to get a task from a task_topic that does not exist");
            None
        }
    } else {
        warn!("TopicService::get_new_task_for_subscriber tried to lock a poisoned mutex");
        None
    }
}
