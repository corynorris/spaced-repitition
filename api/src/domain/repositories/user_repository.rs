use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    auth::PasswordManager,
    errors::{DomainError, DomainResult},
    models::user::{
        AdminUserRoleUpdateData, AdminUserUpdateData, Role, User, UserChangePasswordData,
        UserCreateData, UserProfileUpdateData,
    },
};
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get a user by their ID
    pub async fn get_by_id(&self, user_id: Uuid) -> DomainResult<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id,
                username,
                email,
                role as "role: Role",
                created_at,
                updated_at
            FROM "user"
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?
        .ok_or_else(|| DomainError::EntityNotFound {
            entity: "user",
            key: "id",
            value: user_id.to_string(),
        })
    }

    /// Get a user by their email
    pub async fn get_by_email(&self, email: &str) -> DomainResult<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id,
                username,
                email,
                role as "role: Role",
                created_at,
                updated_at
            FROM "user"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?
        .ok_or_else(|| DomainError::EntityNotFound {
            entity: "user",
            key: "email",
            value: email.to_string(),
        })
    }

    /// Create a new user
    pub async fn create(&self, data: UserCreateData) -> DomainResult<User> {
        let password_hash = PasswordManager::hash_password(data.password).await?;

        match sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user" (username, email, password_hash, role)
            VALUES ($1, $2, $3, role('user'))
            RETURNING 
                user_id,
                username,
                email,
                role as "role: Role",
                created_at,
                updated_at
            "#,
            data.username,
            data.email,
            password_hash,
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                if let Some(db_error) = e.as_database_error() {
                    match db_error.constraint() {
                        Some("user_username_key") => Err(DomainError::UniqueConstraintViolation {
                            field: "username",
                            value: data.username,
                        }),
                        Some("user_email_key") => Err(DomainError::UniqueConstraintViolation {
                            field: "email",
                            value: data.email,
                        }),
                        _ => Err(DomainError::Database(e.to_string())),
                    }
                } else {
                    Err(DomainError::Database(e.to_string()))
                }
            }
        }
    }

    /// Update a user's own profile
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        data: UserProfileUpdateData,
    ) -> DomainResult<User> {
        match sqlx::query_as!(
            User,
            r#"
            UPDATE "user"
            SET 
                username = COALESCE($1, username),
                email = COALESCE($2, email),
                updated_at = NOW()
            WHERE user_id = $3
            RETURNING 
                user_id,
                username,
                email,
                role as "role: Role",
                created_at,
                updated_at
            "#,
            data.username,
            data.email,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                if let Some(db_error) = e.as_database_error() {
                    match db_error.constraint() {
                        Some("user_username_key") => Err(DomainError::UniqueConstraintViolation {
                            field: "username",
                            value: data.username.unwrap_or_default(),
                        }),
                        Some("user_email_key") => Err(DomainError::UniqueConstraintViolation {
                            field: "email",
                            value: data.email.unwrap_or_default(),
                        }),
                        _ => Err(DomainError::Database(e.to_string())),
                    }
                } else {
                    Err(DomainError::Database(e.to_string()))
                }
            }
        }
    }

    /// Admin update of user profile
    pub async fn admin_update_user(
        &self,
        user_id: Uuid,
        data: AdminUserUpdateData,
    ) -> DomainResult<User> {
        match sqlx::query_as!(
            User,
            r#"
            UPDATE "user"
            SET 
                username = COALESCE($1, username),
                email = COALESCE($2, email),
                updated_at = NOW()
            WHERE user_id = $3
            RETURNING 
                user_id,
                username,
                email,
                role as "role: Role",
                created_at,
                updated_at
            "#,
            data.username,
            data.email,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                if let Some(db_error) = e.as_database_error() {
                    match db_error.constraint() {
                        Some("user_username_key") => Err(DomainError::UniqueConstraintViolation {
                            field: "username",
                            value: data.username.unwrap_or_default(),
                        }),
                        Some("user_email_key") => Err(DomainError::UniqueConstraintViolation {
                            field: "email",
                            value: data.email.unwrap_or_default(),
                        }),
                        _ => Err(DomainError::Database(e.to_string())),
                    }
                } else {
                    Err(DomainError::Database(e.to_string()))
                }
            }
        }
    }

    /// Update user's password
    pub async fn update_password(
        &self,
        user_id: Uuid,
        data: UserChangePasswordData,
    ) -> DomainResult<User> {
        let password_hash = PasswordManager::hash_password(data.new_password).await?;

        sqlx::query_as!(
            User,
            r#"
            UPDATE "user"
            SET 
                password_hash = $1,
                updated_at = NOW()
            WHERE user_id = $2
            RETURNING 
                user_id,
                username,
                email,
                role as "role: Role",
                created_at,
                updated_at
            "#,
            password_hash,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))
    }

    /// Update user's role
    pub async fn update_role(
        &self,
        user_id: Uuid,
        data: AdminUserRoleUpdateData,
    ) -> DomainResult<User> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE "user"
            SET 
                role = $1,
                updated_at = NOW()
            WHERE user_id = $2
            RETURNING 
                user_id,
                username,
                email,
                role as "role: Role",
                created_at,
                updated_at
            "#,
            data.role as Role,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))
    }

    /// Verify a user's login credentials
    pub async fn verify_credentials(&self, email: &str, password: &str) -> DomainResult<User> {
        let user = sqlx::query!(
            r#"
            SELECT 
                user_id,
                username,
                email,
                password_hash,
                role as "role: Role",
                created_at,
                updated_at
            FROM "user"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?
        .ok_or_else(|| DomainError::InvalidCredentials)?;

        // Verify the password
        PasswordManager::verify_password(password.to_string(), user.password_hash)
            .await
            .map_err(|_| DomainError::InvalidCredentials)?;

        Ok(User {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    /// Delete a user
    pub async fn delete(&self, user_id: Uuid) -> DomainResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM "user"
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::EntityNotFound {
                entity: "user",
                key: "id",
                value: user_id.to_string(),
            });
        }

        Ok(())
    }
}
