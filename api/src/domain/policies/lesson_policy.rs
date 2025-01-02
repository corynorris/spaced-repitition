use crate::domain::{auth::AuthUser, errors::DomainResult};

pub struct LessonPolicy;

impl LessonPolicy {
    pub fn new() -> Self {
        Self
    }

    pub async fn can_view_lessons(&self, _auth_user: Option<&AuthUser>) -> DomainResult<bool> {
        // Implement your authorization logic here
        Ok(true)
    }
}
