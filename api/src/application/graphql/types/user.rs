use crate::application::graphql::Timestamptz;
use crate::domain::models::user::{
    AdminUserRoleUpdateData, User, UserChangePasswordData, UserCreateData, UserProfileUpdateData,
};
use crate::domain::models::Role;
use async_graphql::*;
use serde::{Deserialize, Serialize};

/// User role enum representing different access levels in the system
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(name = "UserRole")]
pub enum RoleEnum {
    Admin,
    User,
}

impl From<Role> for RoleEnum {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => RoleEnum::Admin,
            Role::User => RoleEnum::User,
        }
    }
}

impl From<RoleEnum> for Role {
    fn from(role: RoleEnum) -> Self {
        match role {
            RoleEnum::Admin => Role::Admin,
            RoleEnum::User => Role::User,
        }
    }
}

/// Core user object returned in GraphQL responses
#[derive(SimpleObject, Clone)]
#[graphql(name = "User")]
pub struct UserObject {
    pub user_id: ID,
    pub username: String,
    pub email: String,
    pub role: RoleEnum,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
}

/// Input for registering a new user
#[derive(InputObject, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Input for user authentication
#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

/// Input for users updating their own profile
#[derive(InputObject)]
pub struct UpdateProfileInput {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Input for changing password
#[derive(InputObject)]
pub struct ChangePasswordInput {
    pub current_password: String,
    pub new_password: String,
}

/// Input for admin updating any user's profile
#[derive(InputObject)]
pub struct AdminUpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Input for admin changing a user's role
#[derive(InputObject)]
pub struct UpdateUserRoleInput {
    pub role: RoleEnum,
}

impl From<CreateUserInput> for UserCreateData {
    fn from(input: CreateUserInput) -> Self {
        Self {
            username: input.username,
            email: input.email,
            password: input.password,
        }
    }
}

impl From<UpdateProfileInput> for UserProfileUpdateData {
    fn from(input: UpdateProfileInput) -> Self {
        Self {
            username: input.username,
            email: input.email,
        }
    }
}

impl From<ChangePasswordInput> for UserChangePasswordData {
    fn from(input: ChangePasswordInput) -> Self {
        Self {
            current_password: input.current_password,
            new_password: input.new_password,
        }
    }
}

impl From<UpdateUserRoleInput> for AdminUserRoleUpdateData {
    fn from(input: UpdateUserRoleInput) -> Self {
        Self {
            role: input.role.into(),
        }
    }
}

/// Response type for authentication operations
#[derive(SimpleObject)]
pub struct AuthPayload {
    pub token: String,
    pub user: UserObject,
}

impl From<User> for UserObject {
    fn from(row: User) -> Self {
        Self {
            user_id: row.user_id.into(),
            username: row.username,
            email: row.email,
            role: row.role.into(),
            created_at: row.created_at.into(),
            updated_at: row.updated_at.map(Into::into),
        }
    }
}
