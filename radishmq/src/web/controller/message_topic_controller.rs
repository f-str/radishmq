use crate::model::message_topic_model::{
    AddPublisherToMessageTopic, AddSubscriberToMessageTopic, CreateMessageTopic,
    PublishToMessageTopic, RemovePublisherFromMessageTopic, RemoveSubscriberFromMessageTopic,
};
use crate::service::message_topic_service;
use crate::utils::types::TopicType;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

pub fn message_topic_controller_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_message_topics)
        .service(get_message_topics)
        .service(create_message_topics)
        .service(delete_message_topics)
        .service(publish_to_message_topics)
        .service(add_subscriber_to_message_topic)
        .service(remove_subscriber_from_message_topic)
        .service(is_there_new_data_for_subscriber)
        .service(get_new_data_for_subscriber);
}

#[get("/message_topics")]
async fn get_all_message_topics() -> impl Responder {
    let topics = message_topic_service::get_all_message_topics().await;
    HttpResponse::Ok().json(topics)
}

#[get("/message_topics/{topic_name}")]
async fn get_message_topics(topic_name: web::Path<String>) -> impl Responder {
    let topic = message_topic_service::get_message_topic(topic_name.into_inner()).await;

    match topic {
        Some(topic) => HttpResponse::Ok().json(topic),
        None => HttpResponse::NotFound().body(""),
    }
}

#[post("/message_topics")]
async fn create_message_topics(body: web::Json<CreateMessageTopic>) -> impl Responder {
    let topic = message_topic_service::create_message_topic(body.into_inner()).await;

    match topic {
        Some(topic) => HttpResponse::Created().json(topic),
        None => HttpResponse::Conflict().body(""),
    }
}

#[delete("/message_topics/{topic_name}")]
async fn delete_message_topics(topic_name: web::Path<String>) -> impl Responder {
    let topic = message_topic_service::delete_message_topic(topic_name.into_inner()).await;

    match topic {
        Some(_) => HttpResponse::NoContent().body(""),
        None => HttpResponse::NotFound().body(""),
    }
}

#[post("/message_topics/{topic_name}/publisher")]
async fn add_publisher_to_message_topic(
    topic_name: web::Path<String>,
    body: web::Json<AddPublisherToMessageTopic>,
) -> impl Responder {
    message_topic_service::add_publisher_to_message_topic(
        topic_name.into_inner(),
        body.into_inner(),
    )
    .await;

    HttpResponse::NoContent().body("")
}

#[delete("/message_topics/{topic_name}/publisher/{identifier}")]
async fn remove_publisher_from_message_topic(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
) -> impl Responder {
    message_topic_service::remove_publisher_from_message_topic(
        topic_name.into_inner(),
        RemovePublisherFromMessageTopic::new(identifier.into_inner()),
    )
    .await;

    HttpResponse::NoContent().body("")
}

#[post("/message_topics/{topic_name}/publisher/{identifier}/publish")]
async fn publish_to_message_topics(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
    body: web::Json<PublishToMessageTopic<TopicType>>,
) -> impl Responder {
    message_topic_service::publish_to_message_topic(
        topic_name.into_inner(),
        identifier.into_inner(),
        body.into_inner(),
    )
    .await;

    HttpResponse::NoContent().body("")
}

#[post("/message_topics/{topic_name}/subscribers")]
async fn add_subscriber_to_message_topic(
    topic_name: web::Path<String>,
    body: web::Json<AddSubscriberToMessageTopic>,
) -> impl Responder {
    message_topic_service::add_subscriber_to_message_topic(
        topic_name.into_inner(),
        body.into_inner(),
    )
    .await;

    HttpResponse::NoContent().body("")
}

#[delete("/message_topics/{topic_name}/subscribers/{identifier}")]
async fn remove_subscriber_from_message_topic(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
) -> impl Responder {
    message_topic_service::remove_subscriber_from_message_topic(
        topic_name.into_inner(),
        RemoveSubscriberFromMessageTopic::new(identifier.into_inner()),
    )
    .await;

    HttpResponse::NoContent().body("")
}

#[get("/message_topics/{topic_name}/subscribers/{identifier}/is_new_data")]
async fn is_there_new_data_for_subscriber(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
) -> impl Responder {
    let data = message_topic_service::is_there_new_data_for_subscriber(
        topic_name.into_inner(),
        identifier.into_inner(),
    )
    .await;

    match data {
        Some(data) => HttpResponse::Ok().json(data),
        None => HttpResponse::NotFound().body(""),
    }
}

#[get("/message_topics/{topic_name}/subscribers/{identifier}/get_data")]
async fn get_new_data_for_subscriber(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
) -> impl Responder {
    let data = message_topic_service::get_new_data_for_subscriber(
        topic_name.into_inner(),
        identifier.into_inner(),
    )
    .await;

    match data {
        Some(data) => HttpResponse::Ok().json(data),
        None => HttpResponse::NotFound().body(""),
    }
}
