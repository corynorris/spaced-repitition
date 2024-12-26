use crate::http::types::Timestamptz;
use serde::{Deserialize, Serialize};

/// Request/response wrapper for user endpoints
#[derive(Serialize, Deserialize)]
pub struct UserWrapper<T> {
    pub user: T,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub email: String,
    pub token: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamptz>,
}
