mod handlers;
mod types;

use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/api/users", post(handlers::create_user))
        .route("/api/users/login", post(handlers::login_user))
        .route(
            "/api/user",
            get(handlers::get_current_user).patch(handlers::update_user),
        )
}
