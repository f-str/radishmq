use crate::web::controller::message_topic_controller::message_topic_controller_config;
use crate::web::controller::task_topic_controller::task_topic_controller_config;
use actix_web::{middleware, App, HttpServer};
use std::env;

pub async fn start_webserver() -> std::io::Result<()> {
    let port = env::var("HTTP_PORT")
        .expect("HTTP_PORT must be set")
        .parse::<u16>()
        .expect("HTTP_PORT must be a number");

    let address = env::var("HTTP_ADDRESS").expect("HTTP_ADDRESS must be set");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(message_topic_controller_config)
            .configure(task_topic_controller_config)
    })
    .bind((address, port))?
    .run()
    .await
}
