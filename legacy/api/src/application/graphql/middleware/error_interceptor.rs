use async_graphql::*;
use std::sync::Arc;

/// Middleware to ensure consistent error handling across all resolvers
pub struct ErrorMiddleware;

impl ErrorMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl async_graphql::extensions::ExtensionFactory for ErrorMiddleware {
    fn create(&self) -> Arc<dyn async_graphql::extensions::Extension> {
        Arc::new(ErrorInterceptor)
    }
}

struct ErrorInterceptor;

#[async_trait::async_trait]
impl async_graphql::extensions::Extension for ErrorInterceptor {
    async fn resolve(
        &self,
        ctx: &async_graphql::extensions::ExtensionContext<'_>,
        info: async_graphql::extensions::ResolveInfo<'_>,
        next: async_graphql::extensions::NextResolve<'_>,
    ) -> async_graphql::ServerResult<Option<async_graphql::Value>> {
        match next.run(ctx, info).await {
            Ok(value) => Ok(value),
            Err(e) => {
                // Convert any error to our GraphQLError first
                if e.extensions
                    .as_ref()
                    .and_then(|ext| ext.get("code"))
                    .is_some()
                {
                    // If it's already a GraphQLError, pass it through
                    Err(e)
                } else {
                    // For any other error, return a generic internal error
                    // We log the original error for debugging
                    log::error!("Unhandled error in GraphQL resolver: {:?}", e);
                    Err(async_graphql::ServerError::new(
                        "An internal error occurred",
                        None,
                    ))
                }
            }
        }
    }
}

/// Helper trait to extend Schema with error handling
pub trait SchemaErrorHandler {
    fn enable_error_handling(self) -> Self;
}

impl<Query, Mutation, Subscription> SchemaErrorHandler
    for SchemaBuilder<Query, Mutation, Subscription>
where
    Query: async_graphql::ObjectType + 'static,
    Mutation: async_graphql::ObjectType + 'static,
    Subscription: async_graphql::SubscriptionType + 'static,
{
    fn enable_error_handling(self) -> Self {
        self.extension(ErrorMiddleware::new())
    }
}
