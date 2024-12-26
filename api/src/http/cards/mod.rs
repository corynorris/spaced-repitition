mod handlers;
mod types;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/card-types",
            get(handlers::get_card_types).post(handlers::create_card_type),
        )
        .route("/api/cards", post(handlers::create_card))
        .route("/api/cards/:card_id", get(handlers::get_card))
}
