use crate::config::Config;
use axum::{Extension, Router};
use extractor::VerifyingKey;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

// Utility modules.
mod error;
mod extractor;
mod types;
mod users;
mod cards;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Core API state accessible by handler functions.
/// This contains shared application configuration and database pool.
#[derive(Clone)]
struct ApiContext {
    db: PgPool,
    verifying_key: VerifyingKey,
}

/// Starts the server with the given configuration and database connection.
pub async fn start_server(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                db,
                verifying_key: VerifyingKey::new(&config.hmac_key),
            }))
            .layer(TraceLayer::new_for_http()),
    );

    let addr = config.server_addr();
    tracing::info!("Starting server on {}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

/// Defines the application's routes.
fn api_router() -> Router {
    users::router()
        .merge(cards::router())
}
