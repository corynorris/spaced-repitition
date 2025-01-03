use crate::application::graphql::guards::RoleGuard;
use crate::application::graphql::types::{
    AuthPayload, CreateUserInput, LoginInput, RoleEnum, UpdateProfileInput,
};
use crate::domain::auth::AuthUser;
use crate::domain::models::Role;
use crate::{application::graphql::types::user::UserObject, domain::services::UserService};
use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;

use super::ResolverResult;

pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> ResolverResult<AuthPayload> {
        let user_service = ctx.data::<Arc<UserService>>()?;
        let (user, token) = user_service.register(input.into()).await?;

        Ok(AuthPayload {
            token,
            user: user.into(),
        })
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> ResolverResult<AuthPayload> {
        let user_service = ctx.data::<Arc<UserService>>()?;
        let (user, token) = user_service.login(input.email, input.password).await?;

        Ok(AuthPayload {
            token,
            user: user.into(),
        })
    }

    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn update_profile(
        &self,
        ctx: &Context<'_>,
        input: UpdateProfileInput,
    ) -> ResolverResult<UserObject> {
        let auth_user = ctx.data::<AuthUser>()?;
        let user_service = ctx.data::<Arc<UserService>>()?;
        tracing::info!("Updating profile for user: {:?}", auth_user);

        let updated_user = user_service
            .update_own_profile(auth_user, input.into())
            .await?;

        Ok(updated_user.into())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn change_role(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        new_role: RoleEnum,
    ) -> ResolverResult<UserObject> {
        let auth_user = ctx.data::<AuthUser>()?;
        let user_service = ctx.data::<Arc<UserService>>()?;

        let user_id = Uuid::parse_str(&user_id.to_string())
            .map_err(|_| Error::new("Invalid user ID format"))?;

        let updated_user = user_service
            .change_role(auth_user, user_id, new_role.into())
            .await?;

        Ok(updated_user.into())
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn delete_user(&self, ctx: &Context<'_>, user_id: ID) -> ResolverResult<bool> {
        let auth_user = ctx.data::<AuthUser>()?;
        let user_service = ctx.data::<Arc<UserService>>()?;

        let user_id = Uuid::parse_str(&user_id.to_string())
            .map_err(|_| Error::new("Invalid user ID format"))?;

        user_service.delete_user(auth_user, user_id).await?;

        Ok(true)
    }
}
