use crate::graphql::Timestamptz;
use async_graphql::*;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Database row representation of a card type
#[derive(Debug, sqlx::FromRow)]
pub struct CardTypeRow {
    pub card_type_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub schema: JsonValue,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
}

/// GraphQL representation of a card type
#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct CardType {
    pub card_type_id: ID,
    pub name: String,
    pub description: Option<String>,
    pub schema: JsonValue,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
}

/// Input type for creating a new card type
#[derive(Debug, InputObject)]
pub struct CreateCardTypeInput {
    pub name: String,
    pub description: Option<String>,
    /// JSON schema as a string - will be validated before saving
    pub schema: String,
}

/// Input type for updating an existing card type
#[derive(Debug, InputObject)]
pub struct UpdateCardTypeInput {
    pub name: Option<String>,
    pub description: Option<String>,
    /// JSON schema as a string - will be validated before saving
    pub schema: Option<String>,
}

impl From<CardTypeRow> for CardType {
    fn from(row: CardTypeRow) -> Self {
        Self {
            card_type_id: row.card_type_id.into(),
            name: row.name,
            description: row.description,
            schema: row.schema,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[ComplexObject]
impl CardType {
    /// Get all cards that use this type
    async fn cards(&self, _ctx: &Context<'_>) -> Result<Vec<super::Card>> {
        // Will be implemented with DataLoader
        todo!()
    }

    /// Get the schema as a formatted JSON string
    async fn schema_string(&self) -> String {
        serde_json::to_string_pretty(&self.schema).unwrap_or_default()
    }
}
