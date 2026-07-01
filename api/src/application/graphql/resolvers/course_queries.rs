use crate::application::graphql::types::course::{CourseObject, CourseSummaryObject};
use crate::domain::auth::AuthUser;
use crate::domain::services::CourseService;
use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;

use super::ResolverResult;

#[derive(Default)]
pub struct CourseQuery;

#[Object]
impl CourseQuery {
    /// Get a single course by ID
    async fn course(&self, ctx: &Context<'_>, id: ID) -> ResolverResult<CourseObject> {
        let auth_user = ctx.data::<Option<AuthUser>>().ok().cloned().flatten();
        let course_service = ctx.data::<Arc<CourseService>>()?;
        let course_id = Uuid::parse_str(&id.to_string())
            .map_err(|_| Error::new("Invalid course ID format"))?;

        let course = course_service.get_course(auth_user.as_ref(), course_id).await?;
        Ok(course.into())
    }

    /// List courses for a specific user
    async fn list_user_courses(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
    ) -> ResolverResult<Vec<CourseSummaryObject>> {
        let auth_user = ctx.data::<Option<AuthUser>>().ok().cloned().flatten();
        let course_service = ctx.data::<Arc<CourseService>>()?;
        let user_id = Uuid::parse_str(&user_id.to_string())
            .map_err(|_| Error::new("Invalid user ID format"))?;

        let courses = course_service
            .list_user_courses(auth_user.as_ref(), user_id)
            .await?;
        Ok(courses.into_iter().map(Into::into).collect())
    }

    /// List courses owned by the currently authenticated user
    async fn my_courses(
        &self,
        ctx: &Context<'_>,
    ) -> ResolverResult<Vec<CourseSummaryObject>> {
        let auth_user = ctx.data::<AuthUser>()?;
        let course_service = ctx.data::<Arc<CourseService>>()?;

        let courses = course_service
            .list_user_courses(Some(auth_user), auth_user.user_id)
            .await?;
        Ok(courses.into_iter().map(Into::into).collect())
    }

    /// Search published courses
    async fn search_courses(
        &self,
        ctx: &Context<'_>,
        #[graphql(default)] query: Option<String>,
        #[graphql(default = 20)] limit: Option<i32>,
        #[graphql(default = 0)] offset: Option<i32>,
    ) -> ResolverResult<Vec<CourseSummaryObject>> {
        let course_service = ctx.data::<Arc<CourseService>>()?;

        let courses = course_service
            .search_published_courses(query, limit, offset)
            .await?;
        Ok(courses.into_iter().map(Into::into).collect())
    }
}
