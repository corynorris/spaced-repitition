use crate::application::graphql::guards::RoleGuard;
use crate::application::graphql::types::{
    AuthPayload, CreateUserInput, LoginInput, UpdateProfileInput,
};
use crate::domain::auth::AuthUser;
use crate::domain::models::Role;
use crate::{application::graphql::types::user::UserObject, domain::services::UserService};
use async_graphql::*;
use std::sync::Arc;

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

    #[graphql(guard = "RoleGuard::with_roles(vec![Role::User, Role::Admin])")]
    async fn update_profile(
        &self,
        ctx: &Context<'_>,
        input: UpdateProfileInput,
    ) -> ResolverResult<UserObject> {
        let auth_user = ctx.data::<AuthUser>()?;
        let user_service = ctx.data::<Arc<UserService>>()?;

        let updated_user = user_service
            .update_own_profile(auth_user, input.into())
            .await?;

        Ok(updated_user.into())
    }
}
