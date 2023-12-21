use crate::model::task_topic_model::{
    AddPublisherToTaskTopic, AddSubscriberToTaskTopic, CreateTaskTopic, PublishToTaskTopic,
    RemovePublisherFromTaskTopic, RemoveSubscriberFromTaskTopic,
};
use crate::service::task_topic_service;
use crate::utils::types::TopicType;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

pub fn task_topic_controller_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_task_topics)
        .service(get_task_topics)
        .service(create_task_topics)
        .service(delete_task_topics)
        .service(publish_to_task_topics)
        .service(add_subscriber_to_task_topic)
        .service(remove_subscriber_from_task_topic)
        .service(is_there_a_task_for_subscriber)
        .service(get_new_task_for_subscriber);
}

#[get("/task_topics")]
async fn get_all_task_topics() -> impl Responder {
    let topics = task_topic_service::get_all_task_topics().await;
    HttpResponse::Ok().json(topics)
}

#[get("/task_topics/{topic_name}")]
async fn get_task_topics(topic_name: web::Path<String>) -> impl Responder {
    let topic = task_topic_service::get_task_topic(topic_name.into_inner()).await;

    match topic {
        Some(topic) => HttpResponse::Ok().json(topic),
        None => HttpResponse::NotFound().body(""),
    }
}

#[post("/task_topics")]
async fn create_task_topics(body: web::Json<CreateTaskTopic>) -> impl Responder {
    let topic = task_topic_service::create_task_topic(body.into_inner()).await;

    match topic {
        Some(topic) => HttpResponse::Created().json(topic),
        None => HttpResponse::Conflict().body(""),
    }
}

#[delete("/task_topics/{topic_name}")]
async fn delete_task_topics(topic_name: web::Path<String>) -> impl Responder {
    let topic = task_topic_service::delete_task_topic(topic_name.into_inner()).await;

    match topic {
        Some(_) => HttpResponse::NoContent().body(""),
        None => HttpResponse::NotFound().body(""),
    }
}

#[post("/task_topics/{topic_name}/publisher")]
async fn add_publisher_to_task_topic(
    topic_name: web::Path<String>,
    body: web::Json<AddPublisherToTaskTopic>,
) -> impl Responder {
    task_topic_service::add_publisher_to_task_topic(topic_name.into_inner(), body.into_inner())
        .await;

    HttpResponse::NoContent().body("")
}

#[delete("/task_topics/{topic_name}/publisher/{identifier}")]
async fn remove_publisher_from_task_topic(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
) -> impl Responder {
    task_topic_service::remove_publisher_from_task_topic(
        topic_name.into_inner(),
        RemovePublisherFromTaskTopic::new(identifier.into_inner()),
    )
    .await;

    HttpResponse::NoContent().body("")
}

#[post("/task_topics/{topic_name}/publisher/{identifier}/publish")]
async fn publish_to_task_topics(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
    body: web::Json<PublishToTaskTopic<TopicType>>,
) -> impl Responder {
    task_topic_service::publish_to_task_topic(
        topic_name.into_inner(),
        identifier.into_inner(),
        body.into_inner(),
    )
    .await;

    HttpResponse::NoContent().body("")
}

#[post("/task_topics/{topic_name}/subscriber")]
async fn add_subscriber_to_task_topic(
    topic_name: web::Path<String>,
    body: web::Json<AddSubscriberToTaskTopic>,
) -> impl Responder {
    task_topic_service::add_subscriber_to_task_topic(topic_name.into_inner(), body.into_inner())
        .await;

    HttpResponse::NoContent().body("")
}

#[delete("/task_topics/{topic_name}/subscriber/{identifier}")]
async fn remove_subscriber_from_task_topic(
    topic_name: web::Path<String>,
    body: web::Json<RemoveSubscriberFromTaskTopic>,
) -> impl Responder {
    task_topic_service::remove_subscriber_from_task_topic(
        topic_name.into_inner(),
        body.into_inner(),
    )
    .await;

    HttpResponse::NoContent().body("")
}

#[get("/task_topics/{topic_name}/subscribers/{identifier}/is_there_a_task")]
async fn is_there_a_task_for_subscriber(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
) -> impl Responder {
    let new_data = task_topic_service::is_there_a_task_for_subscriber(
        topic_name.into_inner(),
        identifier.into_inner(),
    )
    .await;

    HttpResponse::Ok().json(new_data)
}

#[get("/task_topics/{topic_name}/subscribers/{identifier}/get_new_task")]
async fn get_new_task_for_subscriber(
    topic_name: web::Path<String>,
    identifier: web::Path<String>,
) -> impl Responder {
    let task = task_topic_service::get_new_task_for_subscriber(
        topic_name.into_inner(),
        identifier.into_inner(),
    )
    .await;

    HttpResponse::Ok().json(task)
}
