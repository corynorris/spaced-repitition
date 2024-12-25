use crate::http::error::Error;
use axum::extract::{Extension, FromRequestParts};
use axum::http::request::Parts;

use crate::http::ApiContext;
use async_trait::async_trait;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderValue;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha384;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);
const SCHEME_PREFIX: &str = "Bearer ";

/// A type that holds the HMAC key used for signing and verifying JWTs.
/// This is wrapped in an Arc to avoid cloning the key for each request.
#[derive(Clone)]
pub struct VerifyingKey(Arc<Hmac<Sha384>>);

impl VerifyingKey {
    /// Create a new VerifyingKey from a secret string.
    pub fn new(secret: &str) -> Self {
        let hmac = Hmac::<Sha384>::new_from_slice(secret.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");
        Self(Arc::new(hmac))
    }

    /// Get a reference to the underlying HMAC key.
    fn key(&self) -> &Hmac<Sha384> {
        &self.0
    }
}

/// Add this as a parameter to a handler function to require the user to be logged in.
///
/// Parses a JWT from the `Authorization: Bearer <token>` header.
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
}

/// Add this as a parameter to a handler function to optionally check if the user is logged in.
///
/// If the `Authorization` header is absent then this will be `Self(None)`, otherwise it will
/// validate the token.
#[derive(Clone, Debug)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    /// The subject of the token (user_id)
    sub: String,
    /// The user ID for backwards compatibility and easier access
    user_id: Uuid,
    /// Standard JWT expiration time claim
    exp: i64,
}

impl AuthUser {
    pub(in crate::http) fn to_jwt(&self, ctx: &ApiContext) -> String {
        AuthUserClaims {
            sub: self.user_id.to_string(), // Standard JWT subject claim
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(ctx.verifying_key.key())
        .expect("HMAC signing should be infallible")
    }

    fn from_authorization(ctx: &ApiContext, auth_header: &HeaderValue) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_| {
            log::debug!("Authorization header is not UTF-8");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            log::debug!(
                "Authorization header is using wrong scheme: {:?}",
                auth_header
            );
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt = jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token)
            .map_err(|e| {
                log::debug!(
                    "failed to parse Authorization header {:?}: {}",
                    auth_header,
                    e
                );
                Error::Unauthorized
            })?;

        let jwt = jwt.verify_with_key(ctx.verifying_key.key()).map_err(|e| {
            log::debug!("JWT failed to verify: {}", e);
            Error::Unauthorized
        })?;

        let (_header, claims) = jwt.into();

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("token expired");
            return Err(Error::Unauthorized);
        }

        // Verify that the subject claim matches the user_id
        if claims.sub != claims.user_id.to_string() {
            log::debug!("sub claim does not match user_id");
            return Err(Error::Unauthorized);
        }

        Ok(Self {
            user_id: claims.user_id,
        })
    }
}

impl MaybeAuthUser {
    /// If this is `Self(Some(AuthUser))`, return `AuthUser::user_id`
    pub fn user_id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|auth_user| auth_user.user_id)
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: Extension<ApiContext> = Extension::from_request_parts(parts, state)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        Self::from_authorization(&ctx, auth_header)
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for MaybeAuthUser {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: Extension<ApiContext> = Extension::from_request_parts(parts, state)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        Ok(Self(
            parts
                .headers
                .get(AUTHORIZATION)
                .map(|auth_header| AuthUser::from_authorization(&ctx, auth_header))
                .transpose()?
        ))
    }
}