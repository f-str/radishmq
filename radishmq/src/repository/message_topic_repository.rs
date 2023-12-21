use crate::event_queue::worker::ThreadData;
use crate::model::entity::message_topic_entities::MessageTopicEntity;
use crate::model::message_topic_model::{
    AddPublisherToMessageTopic, AddSubscriberToMessageTopic, CreateMessageTopic,
    PublishToMessageTopic, RemovePublisherFromMessageTopic, RemoveSubscriberFromMessageTopic,
};
use crate::utils::types::TopicType;
use sqlx::{Pool, Postgres};

pub async fn create_message_topic(thread_data: ThreadData, data: CreateMessageTopic) {
    let result = sqlx::query(
        r#"
            INSERT INTO message_topic (topic_name)
            VALUES ($1)
            "#,
    )
    .bind(data.name.clone())
    .execute(&thread_data.db_connection_pool)
    .await;

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Error while creating message topic: {}", e);
        }
    }
}

pub async fn delete_message_topic(thread_data: ThreadData, topic_name: String) {
    let result = sqlx::query(
        r#"
            DELETE FROM message_topic
            WHERE topic_name = $1
            "#,
    )
    .bind(topic_name.clone())
    .execute(&thread_data.db_connection_pool)
    .await;

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Error while deleting message topic: {}", e);
        }
    }
}

async fn get_message_topic_by_name(
    db_connection_pool: &Pool<Postgres>,
    topic_name: String,
) -> Option<MessageTopicEntity> {
    let topic = sqlx::query_as::<_, MessageTopicEntity>(
        r#"
            SELECT * FROM message_topic
            WHERE topic_name = $1
            "#,
    )
    .bind(topic_name.clone())
    .fetch_optional(db_connection_pool)
    .await;

    match topic {
        Ok(topic) => topic,
        Err(e) => {
            println!("Error while fetching the message topic: {}", e);
            None
        }
    }
}

pub async fn publish_to_message_topic(
    thread_data: ThreadData,
    topic_name: String,
    _data: PublishToMessageTopic<TopicType>,
) {
    let topic =
        get_message_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    UPDATE message_topic SET data_index = data_index + 1
                    WHERE id = $1
                    "#,
            )
            .bind(topic.id)
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while publishing to message topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

pub async fn add_publisher_to_message_topic(
    thread_data: ThreadData,
    topic_name: String,
    data: AddPublisherToMessageTopic,
) {
    let topic =
        get_message_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    INSERT INTO message_topic_publisher (message_topic_id, publisher_name)
                    VALUES ($1, $2)
                    "#,
            )
            .bind(topic.id)
            .bind(data.publisher)
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while adding publisher to message topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

pub async fn remove_publisher_from_message_topic(
    thread_data: ThreadData,
    topic_name: String,
    data: RemovePublisherFromMessageTopic,
) {
    let topic =
        get_message_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    DELETE FROM message_topic_publisher
                    WHERE message_topic_id = $1 AND publisher_name = $2
                    "#,
            )
            .bind(topic.id)
            .bind(data.publisher)
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while removing publisher from message topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

pub async fn add_subscriber_to_message_topic(
    thread_data: ThreadData,
    topic_name: String,
    data: AddSubscriberToMessageTopic,
) {
    let topic =
        get_message_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    INSERT INTO message_topic_subscriber (message_topic_id, subscriber_name, subscriber_index)
                    VALUES ($1, $2, $3)
                    "#,
            )
                .bind(topic.id)
                .bind(data.subscriber)
                .bind(topic.data_index)
                .execute(&thread_data.db_connection_pool)
                .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while adding subscriber to message topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

pub async fn remove_subscriber_from_message_topic(
    thread_data: ThreadData,
    topic_name: String,
    data: RemoveSubscriberFromMessageTopic,
) {
    let topic =
        get_message_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    DELETE FROM message_topic_subscriber
                    WHERE message_topic_id = $1 AND subscriber_name = $2
                    "#,
            )
            .bind(topic.id)
            .bind(data.subscriber)
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while removing subscriber from message topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

#[allow(dead_code)]
pub async fn fetch_data_from_message_topic(
    thread_data: ThreadData,
    topic_name: String,
    subscriber: String,
    subscriber_index: usize,
) {
    let topic =
        get_message_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    UPDATE message_topic_subscriber SET subscriber_index = $1
                    WHERE message_topic_id = $2 AND subscriber_name = $3
                    "#,
            )
            .bind(subscriber_index as i64)
            .bind(topic.id)
            .bind(subscriber)
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while updating the subscriber index: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

pub async fn reset_index_of_message_topic(
    thread_data: ThreadData,
    topic_name: String,
    index_subtrahend: usize,
) {
    let topic =
        get_message_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    UPDATE message_topic SET data_index = data_index - $1
                    WHERE id = $2
                    "#,
            )
            .bind(index_subtrahend as i64)
            .bind(topic.id)
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while resetting index of message topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}
