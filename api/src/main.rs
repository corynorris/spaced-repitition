use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use spaced_repetition_api::application::http;
use spaced_repetition_api::infrastructure::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::parse();

    config.setup_logging();

    let db = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    sqlx::migrate!().run(&db).await?;

    http::start_server(config, db).await?;

    Ok(())
}
