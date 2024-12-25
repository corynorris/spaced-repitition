use crate::http::{ApiContext, Result};
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::extract::Extension;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::http::error::{Error, ResultExt};
use crate::http::extractor::AuthUser;
use crate::http::types::Timestamptz;

pub fn router() -> Router {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/login", post(login_user))
        .route("/api/user", 
            get(get_current_user)
            .patch(update_user))
}

/// Request/response wrapper for user endpoints
#[derive(serde::Serialize, serde::Deserialize)]
struct UserWrapper<T> {
    user: T,
}

#[derive(serde::Deserialize)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
struct UpdateUser {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(serde::Serialize)]
struct UserResponse {
    email: String,
    token: String,
    username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<Timestamptz>,
}

async fn create_user(
    Extension(ctx): Extension<ApiContext>,
    Json(UserWrapper { user }): Json<UserWrapper<NewUser>>,
) -> Result<Json<UserWrapper<UserResponse>>> {
    let password_hash = hash_password(user.password).await?;

    let user = sqlx::query!(
        r#"
        insert into "user" (username, email, password_hash)
        values ($1, $2, $3)
        returning user_id, email, username, created_at
        "#,
        user.username,
        user.email,
        password_hash
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(UserWrapper {
        user: UserResponse {
            email: user.email,
            token: AuthUser { user_id: user.user_id }.to_jwt(&ctx),
            username: user.username,
            created_at: Option::from(user.created_at).map(Timestamptz),
        },
    }))
}

async fn login_user(
    Extension(ctx): Extension<ApiContext>,
    Json(UserWrapper { user }): Json<UserWrapper<LoginUser>>,
) -> Result<Json<UserWrapper<UserResponse>>> {
    let login_password = user.password;
    let user = sqlx::query!(
        r#"
        select user_id, email, username, password_hash, created_at
        from "user"
        where email = $1
        "#,
        user.email,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(login_password, user.password_hash).await?;

    Ok(Json(UserWrapper {
        user: UserResponse {
            email: user.email,
            token: AuthUser {
                user_id: user.user_id,
            }
            .to_jwt(&ctx),
            username: user.username,
            created_at: Option::from(user.created_at).map(Timestamptz),
        },
    }))
}

async fn get_current_user(
    auth_user: AuthUser,
    Extension(ctx): Extension<ApiContext>,
) -> Result<Json<UserWrapper<UserResponse>>> {
    let user = sqlx::query!(
        r#"
        select email, username, created_at
        from "user"
        where user_id = $1
        "#,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(UserWrapper {
        user: UserResponse {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
            created_at: Option::from(user.created_at).map(Timestamptz),
        },
    }))
}

async fn update_user(
    auth_user: AuthUser,
    Extension(ctx): Extension<ApiContext>,
    Json(UserWrapper { user }): Json<UserWrapper<UpdateUser>>,
) -> Result<Json<UserWrapper<UserResponse>>> {
    if user == UpdateUser::default() {
        // Need to wrap ctx in Extension again for get_current_user
        return get_current_user(auth_user, Extension(ctx)).await;
    }

    let password_hash = if let Some(password) = user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let user = sqlx::query!(
        r#"
        update "user"
        set email = coalesce($1, email),
            username = coalesce($2, username),
            password_hash = coalesce($3, password_hash)
        where user_id = $4
        returning email, username, created_at
        "#,
        user.email,
        user.username,
        password_hash,
        auth_user.user_id,
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(UserWrapper {
        user: UserResponse {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
            created_at: Option::from(user.created_at).map(Timestamptz),
        },
    }))
}

async fn hash_password(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, &salt)
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")?
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")?
}