use crate::application::graphql::types::course::{
    CourseObject, CreateCourseInput, UpdateCourseInput,
};
use crate::domain::auth::AuthUser;
use crate::domain::services::CourseService;
use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;

use super::ResolverResult;

pub struct CourseMutation;

#[Object]
impl CourseMutation {
    /// Create a new course
    async fn create_course(
        &self,
        ctx: &Context<'_>,
        input: CreateCourseInput,
    ) -> ResolverResult<CourseObject> {
        let auth_user = ctx.data::<AuthUser>()?;
        let course_service = ctx.data::<Arc<CourseService>>()?;

        let data = crate::domain::models::course::CourseCreateData {
            user_id: auth_user.user_id,
            title: input.title,
            description: input.description,
        };

        let course = course_service.create_course(auth_user, data).await?;
        Ok(course.into())
    }

    /// Update an existing course
    async fn update_course(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdateCourseInput,
    ) -> ResolverResult<CourseObject> {
        let auth_user = ctx.data::<AuthUser>()?;
        let course_service = ctx.data::<Arc<CourseService>>()?;
        let course_id = Uuid::parse_str(&id.to_string())
            .map_err(|_| Error::new("Invalid course ID format"))?;

        let data = crate::domain::models::course::CourseUpdateData {
            title: input.title,
            description: input.description,
        };

        let course = course_service
            .update_course(auth_user, course_id, data)
            .await?;
        Ok(course.into())
    }

    /// Publish a course (makes it visible to other users)
    async fn publish_course(&self, ctx: &Context<'_>, id: ID) -> ResolverResult<CourseObject> {
        let auth_user = ctx.data::<AuthUser>()?;
        let course_service = ctx.data::<Arc<CourseService>>()?;
        let course_id = Uuid::parse_str(&id.to_string())
            .map_err(|_| Error::new("Invalid course ID format"))?;

        let course = course_service.publish_course(auth_user, course_id).await?;
        Ok(course.into())
    }

    /// Delete a course
    async fn delete_course(&self, ctx: &Context<'_>, id: ID) -> ResolverResult<bool> {
        let auth_user = ctx.data::<AuthUser>()?;
        let course_service = ctx.data::<Arc<CourseService>>()?;
        let course_id = Uuid::parse_str(&id.to_string())
            .map_err(|_| Error::new("Invalid course ID format"))?;

        course_service.delete_course(auth_user, course_id).await?;
        Ok(true)
    }
}
