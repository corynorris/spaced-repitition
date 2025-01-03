use crate::application::graphql::guards::RoleGuard;
use crate::domain::auth::AuthUser;
use crate::domain::errors::DomainError;
use crate::domain::models::Role;
use crate::{application::graphql::types::user::UserObject, domain::services::UserService};
use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;

use super::ResolverResult;
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Get the currently authenticated user
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn me(&self, ctx: &Context<'_>) -> ResolverResult<Option<UserObject>> {
        let auth_user: &AuthUser = ctx.data::<AuthUser>()?;
        let user_service = ctx.data::<Arc<UserService>>()?;

        let domain_user = user_service
            .get_user(Some(auth_user), auth_user.user_id)
            .await?;

        Ok(Some(UserObject::from(domain_user)))
    }

    /// Get a user by ID (requires authentication)
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn user(&self, ctx: &Context<'_>, id: ID) -> ResolverResult<UserObject> {
        let auth_user: &AuthUser = ctx.data::<AuthUser>()?;
        let user_service = ctx.data::<Arc<UserService>>()?;

        let user_id =
            Uuid::parse_str(&id.to_string()).map_err(|_| Error::new("Invalid user ID format"))?;

        let domain_user = user_service.get_user(Some(auth_user), user_id).await?;

        Ok(UserObject::from(domain_user))
    }
}
