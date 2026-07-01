mod admin_mutations;
mod course_mutations;
mod course_queries;
mod user_mutations;
mod user_queries;

use admin_mutations::AdminMutation;
use async_graphql::*;
use course_mutations::CourseMutation;
use course_queries::CourseQuery;
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

    async fn courses<'ctx>(&self, _ctx: &'ctx Context<'_>) -> CourseQuery {
        CourseQuery::default()
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn users<'ctx>(&self, _ctx: &'ctx Context<'_>) -> UserMutation {
        UserMutation
    }

    async fn courses<'ctx>(&self, _ctx: &'ctx Context<'_>) -> CourseMutation {
        CourseMutation
    }

    async fn admin<'ctx>(&self, _ctx: &'ctx Context<'_>) -> AdminMutation {
        AdminMutation
    }
}
