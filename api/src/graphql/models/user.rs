use crate::{auth::UserRole, graphql::Timestamptz};
use async_graphql::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// User object returned in GraphQL responses
#[derive(SimpleObject, Clone)]
pub struct User {
    pub user_id: ID,
    pub username: String,
    pub email: String,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
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

/// Database row representation of a user
pub struct UserRow {
    pub user_id: Uuid,
    pub role: UserRole,
    pub username: String,
    pub email: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            user_id: row.user_id.into(),
            username: row.username,
            email: row.email,
            created_at: row.created_at.into(),
            updated_at: row.updated_at.map(Into::into),
        }
    }
}
