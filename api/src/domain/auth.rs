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

use crate::domain::{
    errors::{DomainError, DomainResult},
    models::user::Role,
};

pub const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub role: Role,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: Uuid,
    exp: i64,
    role: Role,
}

#[derive(Clone)]
pub struct AuthKey(Arc<Hmac<Sha384>>);

impl AuthKey {
    pub fn new(secret: &str) -> DomainResult<Self> {
        let hmac = Hmac::<Sha384>::new_from_slice(secret.as_bytes()).map_err(|e| {
            DomainError::BusinessRuleViolation(format!("Failed to create HMAC key: {}", e))
        })?;
        Ok(Self(Arc::new(hmac)))
    }

    fn key(&self) -> &Hmac<Sha384> {
        &self.0
    }
}

#[derive(Clone)]
pub struct PasswordManager;

impl PasswordManager {
    pub async fn hash_password(password: String) -> DomainResult<String> {
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(rand::thread_rng());
            PasswordHash::generate(Argon2::default(), password, &salt)
                .map(|hash| hash.to_string())
                .map_err(|e| {
                    DomainError::BusinessRuleViolation(format!("Failed to hash password: {}", e))
                })
        })
        .await
        .map_err(|_| {
            DomainError::BusinessRuleViolation("Password hashing task failed".to_string())
        })?
    }

    pub async fn verify_password(password: String, password_hash: String) -> DomainResult<()> {
        tokio::task::spawn_blocking(move || {
            let hash = PasswordHash::new(&password_hash).map_err(|e| {
                DomainError::BusinessRuleViolation(format!("Invalid password hash: {}", e))
            })?;

            hash.verify_password(&[&Argon2::default()], password)
                .map_err(|e| match e {
                    argon2::password_hash::Error::Password => DomainError::InvalidCredentials,
                    _ => DomainError::BusinessRuleViolation(format!(
                        "Password verification failed: {}",
                        e
                    )),
                })
        })
        .await
        .map_err(|_| {
            DomainError::BusinessRuleViolation("Password verification task failed".to_string())
        })?
    }
}

impl AuthUser {
    pub fn create_token(&self, key: &AuthKey) -> DomainResult<String> {
        let claims = Claims {
            sub: self.user_id.to_string(),
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
            role: self.role,
        };

        claims
            .sign_with_key(key.key())
            .map_err(|e| DomainError::BusinessRuleViolation(format!("Failed to sign JWT: {}", e)))
    }

    pub fn from_token(token: &str, key: &AuthKey) -> DomainResult<Self> {
        let claims: Claims = token.verify_with_key(key.key()).map_err(|e| {
            log::debug!("Failed to verify token: {}", e);
            DomainError::BusinessRuleViolation("Invalid authentication token".to_string())
        })?;

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("Token expired");
            return Err(DomainError::BusinessRuleViolation(
                "Authentication token has expired".to_string(),
            ));
        }

        if claims.sub != claims.user_id.to_string() {
            log::debug!("Subject claim doesn't match user_id");
            return Err(DomainError::BusinessRuleViolation(
                "Invalid token subject".to_string(),
            ));
        }

        Ok(Self {
            user_id: claims.user_id,
            role: claims.role,
        })
    }
}
