use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::errors::{DomainError, DomainResult};

/// Core domain card entity matching the database schema
#[derive(Debug, Clone)]
pub struct Card {
    pub card_id: Uuid,
    pub lesson_id: Uuid,
    pub card_type_id: Uuid,
    pub content: serde_json::Value,
    pub order_index: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

/// Input for creating a new card
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardCreateData {
    pub lesson_id: Uuid,
    pub card_type_id: Uuid,
    pub content: serde_json::Value,
    pub order_index: i32,
}

/// Input for updating an existing card
#[derive(Debug, Clone, Default)]
pub struct CardUpdateData {
    pub content: Option<serde_json::Value>,
    pub order_index: Option<i32>,
}

impl Card {
    /// Validate new card data
    pub fn validate_new(data: &CardCreateData) -> DomainResult<()> {
        let mut errors: HashMap<Cow<str>, Vec<Cow<str>>> = HashMap::new();

        if data.order_index < 0 {
            errors.insert(
                Cow::from("order_index"),
                vec![Cow::from("Order index must be non-negative")],
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(DomainError::ValidationError { errors })
        }
    }

    /// Validate card update data
    pub fn validate_update(data: &CardUpdateData) -> DomainResult<()> {
        let mut errors: HashMap<Cow<str>, Vec<Cow<str>>> = HashMap::new();

        if let Some(index) = data.order_index {
            if index < 0 {
                errors.insert(
                    Cow::from("order_index"),
                    vec![Cow::from("Order index must be non-negative")],
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
    fn test_validate_new_card_content_required() {
        let valid_data = CardCreateData {
            lesson_id: Uuid::new_v4(),
            card_type_id: Uuid::new_v4(),
            content: serde_json::Value::String("Front: Hello\nBack: Hola".to_string()),
            order_index: 0,
        };

        assert!(Card::validate_new(&valid_data).is_ok());
    }

    #[test]
    fn test_validate_new_card_negative_order_index() {
        let invalid_data = CardCreateData {
            lesson_id: Uuid::new_v4(),
            card_type_id: Uuid::new_v4(),
            content: serde_json::Value::String("test".to_string()),
            order_index: -1,
        };

        let result = Card::validate_new(&invalid_data);
        assert!(result.is_err());
        if let Err(DomainError::ValidationError { errors }) = result {
            assert!(errors.contains_key("order_index"));
        }
    }

    #[test]
    fn test_validate_card_update_negative_order_index() {
        let invalid_data = CardUpdateData {
            content: None,
            order_index: Some(-5),
        };

        let result = Card::validate_update(&invalid_data);
        assert!(result.is_err());
        if let Err(DomainError::ValidationError { errors }) = result {
            assert!(errors.contains_key("order_index"));
        }
    }

    #[test]
    fn test_validate_card_update_empty_is_ok() {
        let valid_data = CardUpdateData::default();

        assert!(Card::validate_update(&valid_data).is_ok());
    }
}
