use crate::graphql::Timestamptz;
use async_graphql::*;
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use uuid::Uuid;

/// Database row representation of a card
#[derive(Debug, sqlx::FromRow)]
pub struct CardRow {
    pub card_id: Uuid,
    pub lesson_id: Uuid,
    pub card_type_id: Uuid,
    pub content: JsonValue,
    pub order_index: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

/// GraphQL representation of a card
#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Card {
    pub card_id: ID,
    pub lesson_id: ID,
    pub card_type_id: ID,
    pub content: JsonValue,
    pub order_index: i32,
    pub created_at: Timestamptz,
    pub updated_at: Option<Timestamptz>,
}

/// Input type for creating a new card
#[derive(Debug, InputObject)]
pub struct CreateCardInput {
    pub lesson_id: ID,
    pub card_type_id: ID,
    /// Card content as a JSON string - will be validated against the card type's schema
    pub content: String,
    pub order_index: i32,
}

/// Input type for updating an existing card
#[derive(Debug, InputObject)]
pub struct UpdateCardInput {
    /// Card content as a JSON string - will be validated against the card type's schema
    pub content: Option<String>,
    pub order_index: Option<i32>,
}

impl From<CardRow> for Card {
    fn from(row: CardRow) -> Self {
        Self {
            card_id: row.card_id.into(),
            lesson_id: row.lesson_id.into(),
            card_type_id: row.card_type_id.into(),
            content: row.content,
            order_index: row.order_index,
            created_at: row.created_at.into(),
            updated_at: row.updated_at.map(Into::into),
        }
    }
}

#[ComplexObject]
impl Card {
    /// Get the lesson this card belongs to
    async fn lesson(&self, _ctx: &Context<'_>) -> Result<super::Lesson> {
        // Will be implemented with DataLoader
        todo!()
    }

    /// Get the card type that defines this card's structure
    async fn card_type(&self, _ctx: &Context<'_>) -> Result<super::CardType> {
        // Will be implemented with DataLoader
        todo!()
    }

    /// Get the card's content as a formatted JSON string
    async fn content_string(&self) -> String {
        serde_json::to_string_pretty(&self.content).unwrap_or_default()
    }

    // async fn reorder(
    //     &self,
    //     pool: &PgPool,
    //     lesson_id: Uuid,
    //     card_id: Uuid,
    //     new_index: i32,
    // ) -> Result<(), Error> {
    //     sqlx::query!(
    //         r#"
    //         WITH card_to_move AS (
    //             SELECT order_index as old_index
    //             FROM card
    //             WHERE card_id = $1
    //         ),
    //         affected_range AS (
    //             SELECT
    //                 CASE
    //                     WHEN $2 > old_index THEN old_index
    //                     ELSE $2
    //                 END as range_start,
    //                 CASE
    //                     WHEN $2 > old_index THEN $2
    //                     ELSE old_index
    //                 END as range_end
    //             FROM card_to_move
    //         )
    //         UPDATE card
    //         SET order_index =
    //             CASE
    //                 WHEN card_id = $1 THEN $2
    //                 WHEN order_index >= range_start AND order_index <= range_end THEN
    //                     CASE
    //                         WHEN $2 > old_index THEN order_index - 1
    //                         ELSE order_index + 1
    //                     END
    //             END
    //         FROM card_to_move, affected_range
    //         WHERE lesson_id = $3
    //         AND order_index BETWEEN range_start AND range_end
    //         "#,
    //         card_id,
    //         new_index,
    //         lesson_id
    //     )
    //     .execute(pool)
    //     .await?;

    //     Ok(())
    // }
}
