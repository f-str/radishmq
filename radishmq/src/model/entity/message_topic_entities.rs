use sqlx::types::Uuid;
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageTopicEntity {
    pub id: Uuid,
    pub name: String,
    pub data_index: i64,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageTopicSubscriberEntity {
    pub id: Uuid,
    pub message_topic_id: Uuid,
    pub subscriber_name: String,
    pub subscriber_index: i64,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageTopicPublisherEntity {
    pub id: Uuid,
    pub message_topic_id: Uuid,
    pub publisher_name: String,
}
