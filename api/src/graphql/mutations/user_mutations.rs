use crate::{
    auth::{AuthKey, AuthUser, PasswordManager, UserRole},
    error::{Error, ResultExt},
    graphql::{models::user::*, Result, Timestamptz},
};
use async_graphql::*;
use sqlx::PgPool;
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<AuthPayload> {
        let pool = ctx.data::<PgPool>()?;
        let auth_key = ctx.data::<AuthKey>()?;

        let password_hash = PasswordManager::hash_password(input.password).await?;

        let user_row = sqlx::query_as!(
            UserRow,
            r#"
            INSERT INTO "user" (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING 
                user_id,
                role as "role: UserRole",
                email,
                username,
                created_at,
                updated_at
            "#,
            input.username,
            input.email,
            password_hash
        )
        .fetch_one(pool)
        .await
        .on_constraint("user_username_key", |_| Error::UniqueViolation {
            field: "username",
            message: "username taken",
        })
        .on_constraint("user_email_key", |_| Error::UniqueViolation {
            field: "email",
            message: "email taken",
        })?;

        let auth_user = AuthUser {
            user_id: user_row.user_id,
            role: user_row.role,
        };

        Ok(AuthPayload {
            token: auth_user.create_token(auth_key),
            user: user_row.into(),
        })
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let pool = ctx.data::<PgPool>()?;
        let auth_key = ctx.data::<AuthKey>()?;

        let user = sqlx::query!(
            r#"
            SELECT user_id, email, username, password_hash, role as "role: UserRole", created_at, updated_at
            FROM "user"
            WHERE email = $1
            "#,
            input.email,
        )
        .fetch_optional(pool)
        .await?
        .ok_or(Error::not_found("user", Some("email not found".into())))?;

        PasswordManager::verify_password(input.password, user.password_hash).await?;

        let auth_user = AuthUser {
            user_id: user.user_id,
            role: user.role,
        };

        Ok(AuthPayload {
            token: auth_user.create_token(auth_key),
            user: User {
                user_id: user.user_id.into(),
                email: user.email,
                username: user.username,
                created_at: Timestamptz(user.created_at),
                updated_at: Option::from(user.updated_at.map(Timestamptz)),
            },
        })
    }

    async fn update_profile(&self, ctx: &Context<'_>, input: UpdateUserInput) -> Result<User> {
        let auth_user = ctx
            .data::<Option<AuthUser>>()?
            .as_ref()
            .ok_or(Error::Unauthorized)?;
        let pool = ctx.data::<PgPool>()?;

        let password_hash = if let Some(password) = input.password {
            Some(PasswordManager::hash_password(password).await?)
        } else {
            None
        };

        let user = sqlx::query!(
            r#"
            UPDATE "user"
            SET email = coalesce($1, email),
                username = coalesce($2, username),
                password_hash = coalesce($3, password_hash)
            WHERE user_id = $4
            RETURNING email, username, created_at, updated_at
            "#,
            input.email,
            input.username,
            password_hash,
            auth_user.user_id,
        )
        .fetch_one(pool)
        .await
        .on_constraint("user_username_key", |_| Error::UniqueViolation {
            field: "username",
            message: "username taken",
        })
        .on_constraint("user_email_key", |_| Error::UniqueViolation {
            field: "email",
            message: "email taken",
        })?;

        Ok(User {
            user_id: auth_user.user_id.into(),
            email: user.email,
            username: user.username,
            created_at: Timestamptz(user.created_at),
            updated_at: Option::from(user.updated_at.map(Timestamptz)),
        })
    }
}
