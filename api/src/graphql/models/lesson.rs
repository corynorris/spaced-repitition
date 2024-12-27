use crate::graphql::Timestamptz;
use async_graphql::*;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct LessonRow {
    pub lesson_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub order_index: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, SimpleObject, Clone)]
#[graphql(complex)]
pub struct Lesson {
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

impl From<LessonRow> for Lesson {
    fn from(row: LessonRow) -> Self {
        Self {
            lesson_id: row.lesson_id.into(),
            course_id: row.course_id.into(),
            title: row.title,
            order_index: row.order_index,
            created_at: row.created_at.into(),
            updated_at: row.updated_at.map(Into::into),
        }
    }
}

#[ComplexObject]
impl Lesson {
    async fn cards(&self, _ctx: &Context<'_>) -> Result<Vec<super::Card>> {
        todo!()
    }

    async fn course(&self, _ctx: &Context<'_>) -> Result<super::Course> {
        todo!()
    }
}
