mod db;
mod event_queue;
mod model;
mod repository;
mod service;
mod state;
mod topic;
mod utils;
mod web;

use crate::db::migration::run_migrations;
use crate::event_queue::worker::create_event_queue_workers;
use crate::state::State;
use crate::web::server::start_webserver;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref STATE: Arc<State> = Arc::new(State::new());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_migrations()
        .await
        .expect("Cannot run DB migrations: {}");

    create_event_queue_workers().await;

    start_webserver().await
}
