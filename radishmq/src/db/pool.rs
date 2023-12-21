use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

pub async fn initialize_connection_pool() -> Option<Pool<Postgres>> {
    match get_pool().await {
        Ok(pool) => {
            println!("DB connection pool initialized!");
            Some(pool)
        }
        Err(e) => {
            println!("DB connection pool could not be initialized: {}", e);
            None
        }
    }
}

async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let max_connection = env::var("DB_POOL_MAX_CONNECTIONS_THREAD")
        .expect("DB_POOL_MAX_CONNECTIONS_THREAD must be set")
        .parse::<u32>()
        .expect("DB_POOL_MAX_CONNECTIONS_THREAD must be an unsigned number");
    PgPoolOptions::new()
        .max_connections(max_connection)
        .connect(db_url.as_str())
        .await
}
