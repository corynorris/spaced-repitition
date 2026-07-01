use crate::application::graphql::Timestamptz;
use crate::domain::models::course::{Course, CourseSummary};
use async_graphql::*;

#[derive(Debug, SimpleObject)]
#[graphql(name = "Course")]
pub struct CourseObject {
    pub course_id: ID,
    pub user_id: ID,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
}

#[derive(Debug, SimpleObject)]
#[graphql(name = "CourseSummary")]
pub struct CourseSummaryObject {
    pub course_id: ID,
    pub user_id: ID,
    pub title: String,
    pub description: Option<String>,
    pub lesson_count: i64,
    pub total_cards: i64,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
}

#[derive(Debug, InputObject)]
pub struct CreateCourseInput {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, InputObject)]
pub struct UpdateCourseInput {
    pub title: Option<String>,
    pub description: Option<String>,
}

impl From<Course> for CourseObject {
    fn from(domain: Course) -> Self {
        Self {
            course_id: domain.course_id.into(),
            user_id: domain.user_id.into(),
            title: domain.title,
            description: domain.description,
            created_at: domain.created_at.into(),
            updated_at: domain.updated_at.map(Into::into),
        }
    }
}

impl From<CourseSummary> for CourseSummaryObject {
    fn from(domain: CourseSummary) -> Self {
        Self {
            course_id: domain.course_id.into(),
            user_id: domain.user_id.into(),
            title: domain.title,
            description: domain.description,
            lesson_count: domain.lesson_count,
            total_cards: domain.total_cards,
            created_at: domain.created_at.into(),
            updated_at: domain.updated_at.map(Into::into),
        }
    }
}
