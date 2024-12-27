use crate::graphql::{dataloaders::Loaders, Timestamptz};
use async_graphql::*;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CourseRow {
    pub course_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Course {
    pub course_id: ID,
    pub user_id: ID,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CourseSummaryRow {
    pub course_id: Uuid,
    pub title: String,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub lesson_count: i64,
    pub total_cards: i64,
    pub created_at: Timestamptz,
    pub last_updated: Timestamptz,
}

#[derive(Debug, SimpleObject)]
pub struct CourseSummary {
    pub course_id: ID,
    pub title: String,
    pub user_id: ID,
    pub description: Option<String>,
    pub lesson_count: i64,
    pub total_cards: i64,
    pub created_at: Timestamptz,
    pub last_updated: Timestamptz,
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

impl From<CourseRow> for Course {
    fn from(row: CourseRow) -> Self {
        Self {
            course_id: row.course_id.into(),
            user_id: row.user_id.into(),
            title: row.title,
            description: row.description,
            created_at: row.created_at.into(),
            updated_at: row.updated_at.map(Into::into),
        }
    }
}

impl From<CourseSummaryRow> for CourseSummary {
    fn from(row: CourseSummaryRow) -> Self {
        Self {
            course_id: row.course_id.into(),
            user_id: row.user_id.into(),
            title: row.title,
            description: row.description,
            lesson_count: row.lesson_count,
            total_cards: row.total_cards,
            created_at: row.created_at,
            last_updated: row.last_updated,
        }
    }
}

#[ComplexObject]
impl Course {
    async fn lessons(&self, ctx: &Context<'_>) -> Result<Vec<super::Lesson>> {
        let loaders = ctx.data::<Loaders>()?;
        let course_uuid = Uuid::parse_str(&self.course_id)?;
        let lessons_map = loaders.lesson.load_many(vec![course_uuid]).await?;
        Ok(lessons_map.get(&course_uuid).cloned().unwrap_or_default())
    }
}
