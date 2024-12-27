use argon2::{
    password_hash::{PasswordHash, SaltString},
    Argon2,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::Error;

pub const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

/// Core authenticated user type
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub role: UserRole,
}

/// Claims structure for JWT tokens
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: Uuid,
    exp: i64,
    role: UserRole,
}

/// Signing/verification key for JWT tokens
#[derive(Clone)]
pub struct AuthKey(Arc<Hmac<Sha384>>);

impl AuthKey {
    pub fn new(secret: &str) -> Self {
        let hmac = Hmac::<Sha384>::new_from_slice(secret.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");
        Self(Arc::new(hmac))
    }

    fn key(&self) -> &Hmac<Sha384> {
        &self.0
    }
}

/// Password handling functions
pub struct PasswordManager;

impl PasswordManager {
    /// Hash a password using Argon2
    pub async fn hash_password(password: String) -> Result<String, Error> {
        tokio::task::spawn_blocking(move || -> Result<String, Error> {
            let salt = SaltString::generate(rand::thread_rng());
            PasswordHash::generate(Argon2::default(), password, &salt)
                .map(|hash| hash.to_string())
                .map_err(|e| Error::internal_error(format!("failed to hash password: {}", e)))
        })
        .await
        .map_err(|_| Error::internal_error("password hashing task failed"))?
    }

    /// Verify a password against its hash
    pub async fn verify_password(password: String, password_hash: String) -> Result<(), Error> {
        tokio::task::spawn_blocking(move || -> Result<(), Error> {
            let hash = PasswordHash::new(&password_hash)
                .map_err(|e| Error::internal_error(format!("invalid password hash: {}", e)))?;

            hash.verify_password(&[&Argon2::default()], password)
                .map_err(|e| match e {
                    argon2::password_hash::Error::Password => Error::InvalidCredentials,
                    _ => Error::internal_error(format!("password verification failed: {}", e)),
                })
        })
        .await
        .map_err(|_| Error::internal_error("password verification task failed"))?
    }
}

impl AuthUser {
    /// Create a JWT token for this user
    pub fn create_token(&self, key: &AuthKey) -> String {
        let claims = Claims {
            sub: self.user_id.to_string(),
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
            role: self.role.clone(),
        };

        claims
            .sign_with_key(key.key())
            .expect("HMAC signing should be infallible")
    }

    /// Validate a JWT token and extract the AuthUser
    pub fn from_token(token: &str, key: &AuthKey) -> Result<Self, Error> {
        let jwt = jwt::Token::<jwt::Header, Claims, _>::parse_unverified(token).map_err(|e| {
            log::debug!("Failed to parse token: {}", e);
            Error::InvalidToken
        })?;

        let jwt = jwt.verify_with_key(key.key()).map_err(|e| {
            log::debug!("JWT failed to verify: {}", e);
            Error::InvalidToken
        })?;

        let (_header, claims) = jwt.into();

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("Token expired");
            return Err(Error::TokenExpired);
        }

        if claims.sub != claims.user_id.to_string() {
            log::debug!("Subject claim doesn't match user_id");
            return Err(Error::InvalidToken);
        }

        Ok(Self {
            user_id: claims.user_id,
            role: claims.role,
        })
    }
}
