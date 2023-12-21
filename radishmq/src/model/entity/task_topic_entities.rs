use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskTopicEntity {
    pub id: Uuid,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskTopicSubscriberEntity {
    pub id: Uuid,
    pub task_topic_id: Uuid,
    pub subscriber_name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskTopicPublisherEntity {
    pub id: Uuid,
    pub task_topic_id: Uuid,
    pub publisher_name: String,
}
