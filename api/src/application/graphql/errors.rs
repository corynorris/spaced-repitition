use crate::domain::errors::DomainError;
use async_graphql::{Error as AsyncGraphlError, ErrorExtensions, Value as GraphQLValue};
use std::{borrow::Cow, collections::HashMap};

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
            // Keep validation errors detailed since they help users fix their input
            DomainError::ValidationError { errors } => create_validation_error(errors),
            DomainError::UniqueConstraintViolation { field, value } => {
                let mut errors = HashMap::new();
                errors.insert(
                    Cow::from(field),
                    vec![Cow::from(format!("'{}' already exists", value))],
                );
                create_validation_error(errors)
            }

            // For authorization errors, keep it simple but clear
            DomainError::InsufficientPermissions {
                action: _,
                resource: _,
            } => AsyncGraphlError::new("Not authorized to perform this action").extend_with(
                |_, e| {
                    e.set("code", "FORBIDDEN");
                },
            ),

            // For not found errors, be generic to avoid information disclosure
            DomainError::EntityNotFound { entity, .. } => {
                AsyncGraphlError::new(format!("{} not found", entity)).extend_with(|_, e| {
                    e.set("code", "NOT_FOUND");
                })
            }

            DomainError::InvalidCredentials => AsyncGraphlError::new("Invalid credentials")
                .extend_with(|_, e| {
                    e.set("code", "UNAUTHORIZED");
                }),

            // Hide database details from clients
            DomainError::Database(_) => AsyncGraphlError::new("An internal error occurred")
                .extend_with(|_, e| {
                    e.set("code", "INTERNAL_ERROR");
                }),

            // For business rule violations, keep the message but add a specific code
            DomainError::BusinessRuleViolation(message) => AsyncGraphlError::new(message)
                .extend_with(|_, e| {
                    e.set("code", "BUSINESS_RULE_VIOLATION");
                }),

            // Handle new error types with appropriate generic messages
            DomainError::InvalidStateTransition { .. } => {
                AsyncGraphlError::new("Invalid operation for current state").extend_with(|_, e| {
                    e.set("code", "INVALID_STATE");
                })
            }

            DomainError::CryptographyError(_) => {
                AsyncGraphlError::new("An internal error occurred").extend_with(|_, e| {
                    e.set("code", "INTERNAL_ERROR");
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
        e.set("code", "VALIDATION_ERROR");
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
