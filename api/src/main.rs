use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use spaced_repetition_api::config::Config;
use spaced_repetition_api::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // Parse our configuration from the environment.
    let config = Config::parse();
    
    // Setup logging
    config.setup_logging();

    // Rest of your initialization code...
    let db = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    sqlx::migrate!().run(&db).await?;

    http::start_server(config, db).await?;

    Ok(())
}