use super::{MutationRoot, QueryRoot};
use crate::error::Error;
use async_graphql::{EmptySubscription, Schema};

pub type Result<T> = std::result::Result<T, Error>;
pub type SpacedRepetitionSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(db: sqlx::PgPool) -> SpacedRepetitionSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db)
    .finish()
}
