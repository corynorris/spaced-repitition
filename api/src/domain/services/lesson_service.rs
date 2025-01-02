use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    errors::DomainResult, models::lesson::Lesson, policies::LessonPolicy,
    repositories::LessonRepository,
};

pub struct LessonService {
    repo: Arc<LessonRepository>,
    policy: LessonPolicy,
}

impl LessonService {
    pub fn new(repo: Arc<LessonRepository>) -> Self {
        Self {
            repo,
            policy: LessonPolicy::new(),
        }
    }

    pub async fn get_lessons_by_courses(&self, course_ids: &[Uuid]) -> DomainResult<Lesson> {
        self.repo.get_lessons_by_courses(course_ids).await
    }
}
