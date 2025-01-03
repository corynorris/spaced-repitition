use super::{
    middleware::error_interceptor::SchemaErrorHandler,
    resolvers::{MutationRoot, QueryRoot},
};
use async_graphql::{EmptySubscription, Schema};
pub type SpacedRepetitionSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(db: sqlx::PgPool) -> SpacedRepetitionSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .enable_error_handling()
    .data(db)
    .finish()
}
