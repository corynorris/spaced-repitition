mod user_mutations; // import the user_mutations module+
use async_graphql::*;
pub use user_mutations::UserMutation; // re-export UserMutation

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn users<'ctx>(&self, _ctx: &'ctx Context<'_>) -> UserMutation {
        UserMutation
    }
}
