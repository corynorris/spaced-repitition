use async_graphql::dataloader::*;
use async_graphql::{FieldError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::Lesson;
use crate::domain::services::LessonService;

pub struct LessonDataLoader {
    service: Arc<LessonService>,
}

impl LessonDataLoader {
    pub fn new(service: Arc<LessonService>) -> Self {
        Self { service }
    }
}

impl Loader<Uuid> for LessonDataLoader {
    type Value = Vec<Lesson>;
    type Error = FieldError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let domain_lessons = self.service.get_lessons_by_courses(keys).await?;

        // Group lessons by course_id
        let mut lesson_map: HashMap<Uuid, Vec<Lesson>> = HashMap::new();
        for lesson in domain_lessons {
            lesson_map.entry(lesson.course_id).or_default().push(lesson);
        }

        Ok(lesson_map)
    }
}
