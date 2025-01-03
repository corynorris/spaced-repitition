use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::errors::{DomainError, DomainResult};

/// User roles
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }
}

/// Core domain user entity
#[derive(Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

/// Input for creating a new user in the domain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCreateData {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Input for user updating their own profile
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserProfileUpdateData {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Input for changing a user's password
#[derive(Debug, Clone, PartialEq)]
pub struct UserChangePasswordData {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug)]
pub struct AdminUserSearchFilters {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Input for admin updating a user's role
#[derive(Debug)]
pub struct AdminUserRoleUpdateData {
    pub role: Role,
}

/// Subset of user data that can be publicly shared
#[derive(Debug, Clone)]
pub struct PublicUser {
    pub user_id: Uuid,
    pub username: String,
    pub created_at: OffsetDateTime,
}

impl User {
    pub fn to_public(&self) -> PublicUser {
        PublicUser {
            user_id: self.user_id,
            username: self.username.clone(),
            created_at: self.created_at,
        }
    }

    /// Validate a new user's data
    pub fn validate_new(data: &UserCreateData) -> DomainResult<()> {
        let mut errors = HashMap::new();

        if data.username.len() < 3 {
            errors.insert(
                Cow::from("username"),
                vec![Cow::from("Username must be at least 3 characters")],
            );
        }

        if !Self::is_valid_email(&data.email) {
            errors.insert(Cow::from("email"), vec![Cow::from("Invalid email format")]);
        }

        if data.password.len() < 8 {
            errors.insert(
                Cow::from("password"),
                vec![Cow::from("Password must be at least 8 characters")],
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError { errors })
        }
    }

    /// Validate profile update data
    pub fn validate_profile_update(data: &UserProfileUpdateData) -> DomainResult<()> {
        let mut errors = HashMap::new();

        if let Some(username) = &data.username {
            if username.len() < 3 {
                errors.insert(
                    Cow::from("username"),
                    vec![Cow::from("Username must be at least 3 characters")],
                );
            }
        }

        if let Some(email) = &data.email {
            if !Self::is_valid_email(email) {
                errors.insert(Cow::from("email"), vec![Cow::from("Invalid email format")]);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError { errors })
        }
    }

    /// Validate password change data
    pub fn validate_password_change(data: &UserChangePasswordData) -> DomainResult<()> {
        let mut errors = HashMap::new();

        if data.new_password.len() < 8 {
            errors.insert(
                Cow::from("new_password"),
                vec![Cow::from("Password must be at least 8 characters")],
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError { errors })
        }
    }

    /// Validate email format
    fn is_valid_email(email: &str) -> bool {
        // Basic email validation
        // TODO: Consider using a proper email validation crate
        email.contains('@') && email.contains('.')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_new_user() {
        let valid_data = UserCreateData {
            username: "johndoe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
        };

        assert!(User::validate_new(&valid_data).is_ok());

        let invalid_data = UserCreateData {
            username: "jo".to_string(),
            email: "invalid-email".to_string(),
            password: "short".to_string(),
        };

        let result = User::validate_new(&invalid_data);
        assert!(result.is_err());
        if let Err(DomainError::ValidationError { errors }) = result {
            assert!(errors.contains_key("username"));
            assert!(errors.contains_key("email"));
            assert!(errors.contains_key("password"));
        }
    }

    #[test]
    fn test_validate_profile_update() {
        let valid_data = UserProfileUpdateData {
            username: Some("newname".to_string()),
            email: Some("new@example.com".to_string()),
        };

        assert!(User::validate_profile_update(&valid_data).is_ok());

        let invalid_data = UserProfileUpdateData {
            username: Some("x".to_string()),
            email: Some("invalid-email".to_string()),
        };

        let result = User::validate_profile_update(&invalid_data);
        assert!(result.is_err());
        if let Err(DomainError::ValidationError { errors }) = result {
            assert!(errors.contains_key("username"));
            assert!(errors.contains_key("email"));
        }
    }
}
