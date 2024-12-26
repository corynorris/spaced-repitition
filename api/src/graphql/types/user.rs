use crate::graphql::Timestamptz;
use async_graphql::*;
use serde::{Deserialize, Serialize};

/// User object returned in GraphQL responses
#[derive(SimpleObject)]
pub struct User {
    pub user_id: ID,
    pub username: String,
    pub email: String,
    pub created_at: Option<Timestamptz>,
}

/// Input for creating a new user
#[derive(InputObject, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Input for user login
#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

/// Input for updating a user's profile
#[derive(InputObject, Default)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

/// Response type for authentication operations
#[derive(SimpleObject)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}
