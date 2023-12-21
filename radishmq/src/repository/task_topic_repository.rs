use crate::event_queue::worker::ThreadData;
use crate::model::entity::task_topic_entities::TaskTopicEntity;
use crate::model::task_topic_model::{
    AddPublisherToTaskTopic, AddSubscriberToTaskTopic, CreateTaskTopic, PublishToTaskTopic,
    RemovePublisherFromTaskTopic, RemoveSubscriberFromTaskTopic,
};
use crate::utils::types::TopicType;
use sqlx::{Pool, Postgres};

pub async fn create_task_topic(thread_data: ThreadData, data: CreateTaskTopic) {
    let result = sqlx::query(
        r#"
            INSERT INTO task_topic (topic_name)
            VALUES ($1)
            "#,
    )
    .bind(data.name.clone())
    .execute(&thread_data.db_connection_pool)
    .await;

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Error while creating task topic: {}", e);
        }
    }
}

pub async fn delete_task_topic(thread_data: ThreadData, topic_name: String) {
    let result = sqlx::query(
        r#"
            DELETE FROM task_topic
            WHERE topic_name = $1
            "#,
    )
    .bind(topic_name.clone())
    .execute(&thread_data.db_connection_pool)
    .await;

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Error while deleting task topic: {}", e);
        }
    }
}

async fn get_task_topic_by_name(
    db_connection_pool: &Pool<Postgres>,
    topic_name: String,
) -> Option<TaskTopicEntity> {
    let topic = sqlx::query_as::<_, TaskTopicEntity>(
        r#"
            SELECT * FROM task_topic
            WHERE topic_name = $1
            "#,
    )
    .bind(topic_name.clone())
    .fetch_optional(db_connection_pool)
    .await;

    match topic {
        Ok(topic) => topic,
        Err(e) => {
            println!("Error while fetching the task topic: {}", e);
            None
        }
    }
}

#[allow(dead_code)]
pub async fn publish_to_task_topic(
    _thread_data: ThreadData,
    _topic_name: String,
    _data: PublishToTaskTopic<TopicType>,
) {
    // does nothing for now
}

pub async fn add_publisher_to_task_topic(
    thread_data: ThreadData,
    topic_name: String,
    data: AddPublisherToTaskTopic,
) {
    let topic = get_task_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    INSERT INTO task_topic_publisher (topic_id, publisher)
                    VALUES ($1, $2)
                    "#,
            )
            .bind(topic.id)
            .bind(data.publisher.clone())
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while adding publisher to task topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

pub async fn remove_publisher_from_task_topic(
    thread_data: ThreadData,
    topic_name: String,
    data: RemovePublisherFromTaskTopic,
) {
    let topic = get_task_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    DELETE FROM task_topic_publisher
                    WHERE topic_id = $1 AND publisher = $2
                    "#,
            )
            .bind(topic.id)
            .bind(data.publisher.clone())
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while removing publisher from task topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

pub async fn add_subscriber_to_task_topic(
    thread_data: ThreadData,
    topic_name: String,
    data: AddSubscriberToTaskTopic,
) {
    let topic = get_task_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    INSERT INTO task_topic_subscriber (topic_id, subscriber)
                    VALUES ($1, $2)
                    "#,
            )
            .bind(topic.id)
            .bind(data.subscriber.clone())
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while adding subscriber to task topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

pub async fn remove_subscriber_from_task_topic(
    thread_data: ThreadData,
    topic_name: String,
    data: RemoveSubscriberFromTaskTopic,
) {
    let topic = get_task_topic_by_name(&thread_data.db_connection_pool, topic_name.clone()).await;

    match topic {
        Some(topic) => {
            let result = sqlx::query(
                r#"
                    DELETE FROM task_topic_subscriber
                    WHERE topic_id = $1 AND subscriber = $2
                    "#,
            )
            .bind(topic.id)
            .bind(data.subscriber.clone())
            .execute(&thread_data.db_connection_pool)
            .await;

            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while removing subscriber from task topic: {}", e);
                }
            }
        }
        None => {
            println!("Topic not found!");
        }
    }
}

#[allow(dead_code)]
pub async fn fetch_task_from_task_topic(
    _thread_data: ThreadData,
    _topic_name: String,
    _subscriber: String,
) {
    // doe nothing for now
}
