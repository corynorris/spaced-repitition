use crate::auth::AuthUser;
use crate::error::Error;
use crate::http::ApiContext;
use async_trait::async_trait;
use axum::extract::{Extension, FromRequestParts};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;

const SCHEME_PREFIX: &str = "Bearer ";

/// Add this as a parameter to a handler function to require the user to be logged in
#[derive(Clone, Debug)]
pub struct RequiredUser(pub AuthUser);

/// Add this as a parameter to a handler function to optionally check if the user is logged in
#[derive(Clone, Debug)]
pub struct OptionalUser(pub Option<AuthUser>);

impl RequiredUser {
    pub fn user_id(&self) -> uuid::Uuid {
        self.0.user_id
    }
}

impl OptionalUser {
    pub fn user_id(&self) -> Option<uuid::Uuid> {
        self.0.as_ref().map(|user| user.user_id)
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for RequiredUser {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: Extension<ApiContext> = Extension::from_request_parts(parts, state)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

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
        let auth_user = AuthUser::from_token(token, &ctx.auth_key)?;

        Ok(Self(auth_user))
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for OptionalUser {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: Extension<ApiContext> = Extension::from_request_parts(parts, state)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        Ok(Self(parts.headers.get(AUTHORIZATION).and_then(
            |auth_header| {
                auth_header
                    .to_str()
                    .ok()
                    .filter(|h| h.starts_with(SCHEME_PREFIX))
                    .map(|h| &h[SCHEME_PREFIX.len()..])
                    .and_then(|token| AuthUser::from_token(token, &ctx.auth_key).ok())
            },
        )))
    }
}
