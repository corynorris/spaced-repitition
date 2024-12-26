use crate::graphql::resolvers::UserMutation;
use async_graphql::*;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn users<'ctx>(&self, ctx: &'ctx Context<'_>) -> UserMutation {
        UserMutation
    }
}
