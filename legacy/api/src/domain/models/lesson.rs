use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::errors::{DomainError, DomainResult};

/// Core domain lesson entity
#[derive(Debug, Clone)]
pub struct Lesson {
    pub lesson_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub order_index: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

/// Input for creating a new lesson in the domain
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLessonData {
    pub course_id: Uuid,
    pub title: String,
    pub order_index: i32,
}

/// Input for updating lesson data in the domain
#[derive(Debug, Default)]
pub struct UpdateLessonData {
    pub title: Option<String>,
    pub order_index: Option<i32>,
}

impl Lesson {
    /// Validate a new lesson's data
    pub fn validate_new(data: &CreateLessonData) -> DomainResult<()> {
        let mut errors = HashMap::new();

        if data.title.is_empty() {
            errors.insert(Cow::from("title"), vec![Cow::from("Title cannot be empty")]);
        }

        if data.order_index < 0 {
            errors.insert(
                Cow::from("order_index"),
                vec![Cow::from("Order index cannot be negative")],
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError { errors })
        }
    }

    /// Validate lesson update data
    pub fn validate_update(data: &UpdateLessonData) -> DomainResult<()> {
        let mut errors = HashMap::new();

        if let Some(title) = &data.title {
            if title.is_empty() {
                errors.insert(Cow::from("title"), vec![Cow::from("Title cannot be empty")]);
            }
        }

        if let Some(order_index) = data.order_index {
            if order_index < 0 {
                errors.insert(
                    Cow::from("order_index"),
                    vec![Cow::from("Order index cannot be negative")],
                );
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError { errors })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_new_lesson() {
        let valid_data = CreateLessonData {
            course_id: Uuid::new_v4(),
            title: "Lesson 1".to_string(),
            order_index: 1,
        };

        assert!(Lesson::validate_new(&valid_data).is_ok());

        let invalid_data = CreateLessonData {
            course_id: Uuid::new_v4(),
            title: "".to_string(),
            order_index: -1,
        };

        let result = Lesson::validate_new(&invalid_data);
        assert!(result.is_err());
        if let Err(DomainError::ValidationError { errors }) = result {
            assert!(errors.contains_key("title"));
            assert!(errors.contains_key("order_index"));
        }
    }

    #[test]
    fn test_validate_update_lesson() {
        let valid_data = UpdateLessonData {
            title: Some("Updated Lesson".to_string()),
            order_index: Some(2),
        };

        assert!(Lesson::validate_update(&valid_data).is_ok());

        let invalid_data = UpdateLessonData {
            title: Some("".to_string()),
            order_index: Some(-1),
        };

        let result = Lesson::validate_update(&invalid_data);
        assert!(result.is_err());
        if let Err(DomainError::ValidationError { errors }) = result {
            assert!(errors.contains_key("title"));
            assert!(errors.contains_key("order_index"));
        }
    }
}
