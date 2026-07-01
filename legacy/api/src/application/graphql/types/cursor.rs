use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CursorInput {
    pub after: Option<String>,
    pub first: Option<i32>,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
}

use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject)]
pub struct CursorResult<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    pub end_cursor: Option<String>,
}
