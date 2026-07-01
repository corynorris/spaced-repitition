use crate::{application::graphql::Timestamptz, domain::models::Lesson};
use async_graphql::*;

#[derive(Debug, Clone, SimpleObject)]
#[graphql(name = "Lesson")] // Keep GraphQL type name as "Lesson"
pub struct LessonObject {
    pub lesson_id: ID,
    pub course_id: ID,
    pub title: String,
    pub order_index: i32,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
}

#[derive(Debug, InputObject)]
pub struct CreateLessonInput {
    pub course_id: ID,
    pub title: String,
    pub order_index: i32,
}

#[derive(Debug, InputObject)]
pub struct UpdateLessonInput {
    pub title: Option<String>,
    pub order_index: Option<i32>,
}

impl From<Lesson> for LessonObject {
    fn from(domain: Lesson) -> Self {
        Self {
            lesson_id: domain.lesson_id.into(),
            course_id: domain.course_id.into(),
            title: domain.title,
            order_index: domain.order_index,
            created_at: domain.created_at.into(),
            updated_at: domain.updated_at.map(Into::into),
        }
    }
}
