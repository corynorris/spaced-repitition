use crate::{
    auth::{AuthUser, UserRole},
    graphql::{models::user::*, Result},
};
use async_graphql::*;
use log::{error, info};
use sqlx::PgPool;

pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get the currently authenticated user
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let auth_user = ctx.data::<Option<AuthUser>>()?;
        let pool = ctx.data::<PgPool>()?;

        match auth_user {
            Some(auth) => {
                info!("Fetching current user with user_id: {}", auth.user_id);

                let user_row = match sqlx::query_as!(
                    UserRow,
                    r#"
                    SELECT 
                        user_id,
                        username,
                        role as "role: UserRole",
                        email,
                        created_at,
                        updated_at
                    FROM "user"
                    WHERE user_id = $1
                    "#,
                    auth.user_id,
                )
                .fetch_one(pool)
                .await
                {
                    Ok(user_row) => user_row,
                    Err(e) => {
                        error!("Failed to fetch user from database: {}", e);
                        return Err(e.into());
                    }
                };

                info!("Successfully fetched user: {}", user_row.username);

                Ok(Some(User::from(user_row)))
            }
            None => {
                info!("No authenticated user found in context");
                Ok(None)
            }
        }
    }
}
