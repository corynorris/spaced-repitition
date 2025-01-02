use crate::application::graphql::{guards::RoleGuard, types::user::*};
use crate::domain::auth::AuthUser;
use crate::domain::{models::Role, services::UserService};
use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserMutation;

pub type GraphQLResult<T> = std::result::Result<T, FieldError>;

#[Object]
impl UserMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> GraphQLResult<AuthPayload> {
        let user_service = ctx.data::<Arc<UserService>>()?;
        let (user, token) = user_service.register(input.into()).await?;

        Ok(AuthPayload {
            token,
            user: user.into(),
        })
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
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
    ) -> Result<UserObject> {
        let auth_user = ctx.data::<AuthUser>()?;
        let user_service = ctx.data::<Arc<UserService>>()?;

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
    ) -> Result<UserObject> {
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
    async fn delete_user(&self, ctx: &Context<'_>, user_id: ID) -> Result<bool> {
        let auth_user = ctx.data::<AuthUser>()?;
        let user_service = ctx.data::<Arc<UserService>>()?;

        let user_id = Uuid::parse_str(&user_id.to_string())
            .map_err(|_| Error::new("Invalid user ID format"))?;

        user_service.delete_user(auth_user, user_id).await?;

        Ok(true)
    }
}
