use axum::{
    body::Body,
    http::{header::WWW_AUTHENTICATE, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use sqlx::error::DatabaseError;
use std::{borrow::Cow, collections::HashMap};

/// Core error type for the application
#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Authentication Errors
    #[error("authentication required")]
    Unauthorized,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("invalid authentication token")]
    InvalidToken,

    #[error("authentication token has expired")]
    TokenExpired,

    // Authorization Errors
    #[error("user may not perform that action")]
    Forbidden,

    // Resource Errors
    #[error("resource not found")]
    NotFound {
        resource: &'static str,
        message: Option<String>,
    },

    // Validation Errors
    #[error("validation error")]
    ValidationError {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    // Database Errors
    #[error("database error")]
    Database(#[from] sqlx::Error),

    // Constraint Errors
    #[error("constraint violation")]
    UniqueViolation {
        field: &'static str,
        message: &'static str,
    },

    // GraphQL Errors
    #[error("graphql error")]
    GraphQL(async_graphql::Error),

    // Internal Errors
    #[error("internal server error")]
    Internal(#[from] anyhow::Error),

    // Internal error with message
    #[error("{0}")]
    InternalWithMessage(String),
}

impl Error {
    /// Create a validation error with the given key-value pairs
    pub fn validation_error<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();
        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }
        Self::ValidationError { errors: error_map }
    }

    /// Create a not found error with an optional message
    pub fn not_found(resource: &'static str, message: Option<String>) -> Self {
        Self::NotFound { resource, message }
    }

    /// Create an internal error with a message
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalWithMessage(message.into())
    }

    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized
            | Self::InvalidCredentials
            | Self::InvalidToken
            | Self::TokenExpired => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::ValidationError { .. } | Self::UniqueViolation { .. } => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            Self::Database(_) | Self::Internal(_) | Self::InternalWithMessage(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::GraphQL(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Get a user-safe error message
    fn public_message(&self) -> String {
        match self {
            Self::Database(_) | Self::Internal(_) => {
                "An internal server error occurred".to_string()
            }
            _ => self.to_string(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        match &self {
            Self::ValidationError { errors } => {
                let errors = serde_json::json!({ "errors": errors });
                (self.status_code(), Json(errors)).into_response()
            }
            Self::UniqueViolation { field, message } => {
                let errors = serde_json::json!({
                    "errors": {
                        *field: [message]
                    }
                });
                (self.status_code(), Json(errors)).into_response()
            }
            _ => {
                let mut response = (self.status_code(), self.public_message()).into_response();

                if matches!(
                    self,
                    Self::Unauthorized | Self::InvalidToken | Self::TokenExpired
                ) {
                    response
                        .headers_mut()
                        .insert(WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
                }

                response
            }
        }
    }
}

/// Helper trait for converting database constraint errors
pub trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Database(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}

impl From<async_graphql::Error> for Error {
    fn from(err: async_graphql::Error) -> Self {
        Error::GraphQL(err)
    }
}
