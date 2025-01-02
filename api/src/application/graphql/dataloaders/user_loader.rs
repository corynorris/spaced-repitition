// use crate::application::graphql::models::User;
// use crate::application::graphql::Timestamptz;
// use async_graphql::dataloader::Loader;
// use async_graphql::{FieldError, Result};
// use sqlx::PgPool;
// use std::collections::HashMap;
// use uuid::Uuid;

// #[derive(Clone)]
// pub struct UserLoader(PgPool);

// impl UserLoader {
//     pub fn new(pool: PgPool) -> Self {
//         Self(pool)
//     }
// }

// impl Loader<Uuid> for UserLoader {
//     type Value = User;
//     type Error = FieldError;

//     async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
//         Ok(sqlx::query_as!(
//             User,
//             r#"
//             SELECT
//                 user_id as "user_id: Uuid",
//                 username,
//                 email,
//                 created_at as "created_at: Timestamptz",
//                 updated_at as "updated_at?: Timestamptz"
//             FROM "user"
//             WHERE user_id = ANY($1)
//             "#,
//             keys
//         )
//         .fetch_all(&self.0)
//         .await?
//         .into_iter()
//         .map(|user| (user.user_id, user))
//         .collect())
//     }
// }
