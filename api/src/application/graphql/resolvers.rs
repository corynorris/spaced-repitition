mod user_mutations;
mod user_queries;

use async_graphql::*;
use user_mutations::UserMutation;
use user_queries::UserQuery;

use super::errors::GraphQLError;

pub type ResolverResult<T> = std::result::Result<T, GraphQLError>;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users<'ctx>(&self, _ctx: &'ctx Context<'_>) -> UserQuery {
        UserQuery
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn users<'ctx>(&self, _ctx: &'ctx Context<'_>) -> UserMutation {
        UserMutation
    }
}
