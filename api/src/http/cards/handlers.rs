use crate::http::types::Timestamptz;
use axum::{
    extract::{Extension, Path},
    Json,
};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::http::{
    error::{Error, ResultExt},
    extractor::AuthUser,
    ApiContext, Result,
};

use super::types::{Card, CardType, CardTypeWrapper, CardWrapper, NewCard, NewCardType};

pub async fn create_card_type(
    _auth_user: AuthUser, // TODO: Add admin check
    Extension(ctx): Extension<ApiContext>,
    Json(CardTypeWrapper { card_type }): Json<CardTypeWrapper<NewCardType>>,
) -> Result<Json<CardTypeWrapper<CardType>>> {
    let card_type = sqlx::query_as!(
        CardType,
        r#"
        INSERT INTO card_type (name, schema)
        VALUES ($1, $2)
        RETURNING type_id, name, schema as "schema: JsonValue",
                  created_at as "created_at?: Timestamptz",
                  updated_at as "updated_at?: Timestamptz"
        "#,
        card_type.name,
        card_type.schema
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("card_type_name_key", |_| {
        Error::unprocessable_entity([("name", "name taken")])
    })?;

    Ok(Json(CardTypeWrapper { card_type }))
}

/// Get all card types
pub async fn get_card_types(Extension(ctx): Extension<ApiContext>) -> Result<Json<Vec<CardType>>> {
    let card_types = sqlx::query_as!(
        CardType,
        r#"
        SELECT type_id, name, schema as "schema: JsonValue",
               created_at as "created_at?: Timestamptz",
               updated_at as "updated_at?: Timestamptz"
        FROM card_type
        ORDER BY name
        "#
    )
    .fetch_all(&ctx.db)
    .await?;

    Ok(Json(card_types))
}

/// Create a new card
pub async fn create_card(
    auth_user: AuthUser,
    Extension(ctx): Extension<ApiContext>,
    Json(CardWrapper { card }): Json<CardWrapper<NewCard>>,
) -> Result<Json<CardWrapper<Card>>> {
    // First verify the card type exists
    let _card_type = sqlx::query!(
        r#"SELECT type_id FROM card_type WHERE type_id = $1"#,
        card.type_id
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(|_| Error::unprocessable_entity([("type_id", "invalid card type")]))?;

    // TODO: Validate card content against type schema

    let card = sqlx::query_as!(
        Card,
        r#"
        INSERT INTO card (type_id, content, is_public, created_by_user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING card_id, type_id, content as "content: JsonValue",
                  is_public, created_by_user_id,
                  created_at as "created_at?: Timestamptz",
                  updated_at as "updated_at?: Timestamptz"
        "#,
        card.type_id,
        card.content,
        card.is_public,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(CardWrapper { card }))
}

/// Get a card by ID
pub async fn get_card(
    Extension(ctx): Extension<ApiContext>,
    Path(card_id): Path<Uuid>,
) -> Result<Json<CardWrapper<Card>>> {
    let card = sqlx::query_as!(
        Card,
        r#"
        SELECT card_id, type_id, content as "content: JsonValue",
               is_public, created_by_user_id,
               created_at as "created_at?: Timestamptz",
               updated_at as "updated_at?: Timestamptz"
        FROM card
        WHERE card_id = $1
        "#,
        card_id
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(CardWrapper { card }))
}
