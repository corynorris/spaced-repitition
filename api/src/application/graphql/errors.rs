use crate::domain::errors::DomainError;
use async_graphql::{Error as AsyncGraphlError, ErrorExtensions, Value as GraphQLValue};
use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    ValidationError,
    Forbidden,
    Unauthorized,
    NotFound,
    InternalError,
    BusinessRuleViolation,
    InvalidState,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ValidationError => "VALIDATION_ERROR",
            Self::Forbidden => "FORBIDDEN",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::NotFound => "NOT_FOUND",
            Self::InternalError => "INTERNAL_ERROR",
            Self::BusinessRuleViolation => "BUSINESS_RULE_VIOLATION",
            Self::InvalidState => "INVALID_STATE",
        }
    }
}

#[derive(Clone)]
pub struct GraphQLError(DomainError);

impl From<DomainError> for GraphQLError {
    fn from(err: DomainError) -> Self {
        GraphQLError(err)
    }
}

impl From<GraphQLError> for AsyncGraphlError {
    fn from(wrapper: GraphQLError) -> Self {
        match wrapper.0 {
            DomainError::ValidationError { errors } => create_validation_error(errors),

            DomainError::UniqueConstraintViolation { field, value } => {
                let mut errors = HashMap::new();
                errors.insert(
                    Cow::from(field),
                    vec![Cow::from(format!("'{}' already exists", value))],
                );
                create_validation_error(errors)
            }

            DomainError::InsufficientPermissions {
                action: _,
                resource: _,
            } => AsyncGraphlError::new("Not authorized to perform this action").extend_with(
                |_, e| {
                    e.set("code", ErrorCode::Forbidden.as_str());
                },
            ),

            DomainError::EntityNotFound { entity, .. } => {
                AsyncGraphlError::new(format!("{} not found", entity)).extend_with(|_, e| {
                    e.set("code", ErrorCode::NotFound.as_str());
                })
            }

            DomainError::InvalidCredentials => AsyncGraphlError::new("Invalid credentials")
                .extend_with(|_, e| {
                    e.set("code", ErrorCode::Unauthorized.as_str());
                }),

            DomainError::Database(_) => AsyncGraphlError::new("An internal error occurred")
                .extend_with(|_, e| {
                    e.set("code", ErrorCode::InternalError.as_str());
                }),

            DomainError::BusinessRuleViolation(message) => AsyncGraphlError::new(message)
                .extend_with(|_, e| {
                    e.set("code", ErrorCode::BusinessRuleViolation.as_str());
                }),

            DomainError::InvalidStateTransition { .. } => {
                AsyncGraphlError::new("Invalid operation for current state").extend_with(|_, e| {
                    e.set("code", ErrorCode::InvalidState.as_str());
                })
            }

            DomainError::CryptographyError(_) => {
                AsyncGraphlError::new("An internal error occurred").extend_with(|_, e| {
                    e.set("code", ErrorCode::InternalError.as_str());
                })
            }
        }
    }
}

fn create_validation_error(
    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
) -> AsyncGraphlError {
    let mut error = AsyncGraphlError::new("Validation error");
    error = error.extend_with(|_, e| {
        e.set("code", ErrorCode::ValidationError.as_str());
        let mut validation_errors = Vec::new();
        for (field, reasons) in errors {
            for reason in reasons {
                validation_errors.push(GraphQLValue::Object(
                    vec![
                        ("field".to_string(), GraphQLValue::String(field.to_string())),
                        (
                            "reason".to_string(),
                            GraphQLValue::String(reason.to_string()),
                        ),
                    ]
                    .into_iter()
                    .map(|(k, v)| (async_graphql::Name::new(k), v))
                    .collect(),
                ));
            }
        }
        e.set("validation_errors", GraphQLValue::List(validation_errors));
    });
    error
}

impl From<AsyncGraphlError> for GraphQLError {
    fn from(err: AsyncGraphlError) -> Self {
        // Convert GraphQL context errors into a business rule violation
        GraphQLError(DomainError::BusinessRuleViolation(format!("{:?}", err)))
    }
}
