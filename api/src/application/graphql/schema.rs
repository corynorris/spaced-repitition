use super::{MutationRoot, QueryRoot};
use async_graphql::{EmptySubscription, Schema};

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
