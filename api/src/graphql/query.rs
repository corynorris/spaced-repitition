use crate::graphql::resolvers::UserQuery;
use async_graphql::*;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users<'ctx>(&self, ctx: &'ctx Context<'_>) -> UserQuery {
        UserQuery
    }
}
