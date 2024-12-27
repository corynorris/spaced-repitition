mod user_queries;

use async_graphql::*;
pub use user_queries::UserQuery; // re-export UserQuery

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users<'ctx>(&self, _ctx: &'ctx Context<'_>) -> UserQuery {
        UserQuery
    }
}
