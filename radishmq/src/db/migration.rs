use crate::utils::types::Error;
use std::env;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn run_migrations() -> Result<(), Error> {
    let enable_migrations: bool = env::var("ENABLE_MIGRATIONS")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);

    if !enable_migrations {
        println!("Skipping DB migrations...");
        return Ok(());
    }

    println!("Running DB migrations...");

    let config = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (mut client, con) = tokio_postgres::connect(config.as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = con.await {
            eprintln!("connection error: {}", e);
        }
    });

    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    for migration in migration_report.applied_migrations() {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    println!("DB migrations finished!");

    Ok(())
}
