use std::{borrow::Cow, collections::HashMap};

#[derive(thiserror::Error, Debug, Clone)]
pub enum DomainError {
    #[error("invalid state transition: {from} -> {to}")]
    InvalidStateTransition { from: String, to: String },

    #[error("business rule violation: {0}")]
    BusinessRuleViolation(String),

    #[error("validation error")]
    ValidationError {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    #[error("entity not found: {entity} with {key}={value}")]
    EntityNotFound {
        entity: &'static str,
        key: &'static str,
        value: String,
    },

    #[error("insufficient permissions for {action} on {resource}")]
    InsufficientPermissions {
        action: &'static str,
        resource: &'static str,
    },

    #[error("database error: {0}")]
    Database(String),

    #[error("unique constraint violation: {field} '{value}' already exists")]
    UniqueConstraintViolation { field: &'static str, value: String },

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("cryptographic error: {0}")]
    CryptographyError(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
