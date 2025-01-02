// application/errors.rs
#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("authentication required")]
    Unauthorized,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("invalid or expired token")]
    InvalidToken,

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("external service error: {service} - {message}")]
    ExternalService {
        service: &'static str,
        message: String,
    },

    #[error("graphql error: {0}")]
    GraphQL(String),

    #[error("internal server error")]
    Internal(#[from] anyhow::Error),
}

// Example application result type
pub type ApplicationResult<T> = Result<T, ApplicationError>;
