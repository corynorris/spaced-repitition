use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::errors::{DomainError, DomainResult};

/// Core domain course entity matching the database schema
#[derive(Debug, Clone)]
pub struct Course {
    pub course_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_published: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

/// Input for creating a new course
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourseCreateData {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

/// Input for updating an existing course
#[derive(Debug, Clone, Default)]
pub struct CourseUpdateData {
    pub title: Option<String>,
    pub description: Option<String>,
}

/// Summarized course information from materialized view
#[derive(Debug, Clone)]
pub struct CourseSummary {
    pub course_id: Uuid,
    pub title: String,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub is_published: bool,
    pub lesson_count: i64,
    pub total_cards: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Course {
    /// Validate new course data
    pub fn validate_new(data: &CourseCreateData) -> DomainResult<()> {
        let mut errors = HashMap::new();

        if data.title.trim().is_empty() {
            errors.insert(Cow::from("title"), vec![Cow::from("Title cannot be empty")]);
        }

        if data.title.len() > 200 {
            errors.insert(
                Cow::from("title"),
                vec![Cow::from("Title must be 200 characters or less")],
            );
        }

        if let Some(desc) = &data.description {
            if desc.len() > 2000 {
                errors.insert(
                    Cow::from("description"),
                    vec![Cow::from("Description must be 2000 characters or less")],
                );
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError { errors })
        }
    }

    /// Validate course update data
    pub fn validate_update(data: &CourseUpdateData) -> DomainResult<()> {
        let mut errors = HashMap::new();

        if let Some(title) = &data.title {
            if title.trim().is_empty() {
                errors.insert(Cow::from("title"), vec![Cow::from("Title cannot be empty")]);
            }

            if title.len() > 200 {
                errors.insert(
                    Cow::from("title"),
                    vec![Cow::from("Title must be 200 characters or less")],
                );
            }
        }

        if let Some(desc) = &data.description {
            if desc.len() > 2000 {
                errors.insert(
                    Cow::from("description"),
                    vec![Cow::from("Description must be 2000 characters or less")],
                );
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError { errors })
        }
    }

    /// Convert to summary view (useful for testing/mocking)
    pub fn to_summary(&self) -> CourseSummary {
        CourseSummary {
            course_id: self.course_id,
            title: self.title.clone(),
            user_id: self.user_id,
            description: self.description.clone(),
            is_published: self.is_published,
            lesson_count: 0, // Default values when converting from course
            total_cards: 0,  // These would be populated from the materialized view
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn create_test_course(user_id: Uuid) -> Course {
        Course {
            course_id: Uuid::new_v4(),
            user_id,
            title: "Test Course".to_string(),
            description: Some("Test Description".to_string()),
            is_published: true,
            created_at: OffsetDateTime::now_utc(),
            updated_at: Option::from(OffsetDateTime::now_utc()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_new_course() {
        let valid_data = CourseCreateData {
            user_id: Uuid::new_v4(),
            title: "Valid Title".to_string(),
            description: Some("Valid description".to_string()),
        };

        assert!(Course::validate_new(&valid_data).is_ok());

        let invalid_data = CourseCreateData {
            user_id: Uuid::new_v4(),
            title: "".to_string(),
            description: Some("Valid description".to_string()),
        };

        let result = Course::validate_new(&invalid_data);
        assert!(result.is_err());
        if let Err(DomainError::ValidationError { errors }) = result {
            assert!(errors.contains_key("title"));
        }
    }

    #[test]
    fn test_validate_course_update() {
        let valid_data = CourseUpdateData {
            title: Some("New Title".to_string()),
            description: Some("New description".to_string()),
        };

        assert!(Course::validate_update(&valid_data).is_ok());

        let invalid_data = CourseUpdateData {
            title: Some("".to_string()),
            description: None,
        };

        let result = Course::validate_update(&invalid_data);
        assert!(result.is_err());
        if let Err(DomainError::ValidationError { errors }) = result {
            assert!(errors.contains_key("title"));
        }
    }

    #[test]
    fn test_to_summary_conversion() {
        let course = mock::create_test_course(Uuid::new_v4());
        let summary = course.to_summary();

        assert_eq!(summary.course_id, course.course_id);
        assert_eq!(summary.title, course.title);
        assert_eq!(summary.user_id, course.user_id);
        assert_eq!(summary.description, course.description);
        assert_eq!(summary.is_published, course.is_published);
        assert_eq!(summary.lesson_count, 0);
        assert_eq!(summary.total_cards, 0);
    }
}
