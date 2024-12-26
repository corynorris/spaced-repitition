use crate::http::types::Timestamptz;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::types::Uuid;

/// The type of a card, which defines its schema and validation rules
#[derive(Debug, Serialize, Deserialize)]
pub struct CardType {
    pub type_id: Uuid,
    pub name: String,
    pub schema: JsonValue,
    pub created_at: Option<Timestamptz>,
    pub updated_at: Option<Timestamptz>,
}

/// Request body for creating a new card type
#[derive(Debug, Deserialize)]
pub struct NewCardType {
    pub name: String,
    pub schema: JsonValue,
}

/// A card instance that contains content following its type's schema
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub card_id: Uuid,
    pub type_id: Uuid,
    pub content: JsonValue,
    pub is_public: bool,
    pub created_by_user_id: Uuid,
    pub created_at: Option<Timestamptz>,
    pub updated_at: Option<Timestamptz>,
}

/// Request body for creating a new card
#[derive(Debug, Deserialize)]
pub struct NewCard {
    pub type_id: Uuid,
    pub content: JsonValue,
    pub is_public: bool,
}

/// Request/response wrapper for card endpoints following the API pattern
#[derive(Serialize, Deserialize)]
pub struct CardWrapper<T> {
    pub card: T,
}

/// Request/response wrapper for card type endpoints
#[derive(Serialize, Deserialize)]
pub struct CardTypeWrapper<T> {
    pub card_type: T,
}
