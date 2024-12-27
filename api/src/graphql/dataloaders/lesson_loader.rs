use crate::graphql::models::{Lesson, LessonRow};
use async_graphql::dataloader::*;
use async_graphql::{FieldError, Result};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

pub struct LessonLoader {
    pool: Arc<PgPool>,
}

impl LessonLoader {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for LessonLoader {
    type Value = Vec<Lesson>;
    type Error = FieldError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let lessons = sqlx::query_as!(
            LessonRow,
            r#"
            SELECT 
                lesson_id,
                course_id,
                title,
                order_index,
                created_at,
                updated_at
            FROM lesson
            WHERE course_id = ANY($1)
            ORDER BY order_index ASC
            "#,
            keys
        )
        .fetch_all(&*self.pool)
        .await?;

        // Group lessons by course_id
        let mut lesson_map: HashMap<Uuid, Vec<Lesson>> = HashMap::new();
        for lesson in lessons {
            lesson_map
                .entry(lesson.course_id)
                .or_default()
                .push(lesson.into());
        }

        Ok(lesson_map)
    }
}
