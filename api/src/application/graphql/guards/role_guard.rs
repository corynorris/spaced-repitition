use crate::domain::models::Role;
use crate::errors::AppError;
use crate::infrastructure::auth::AuthUser;
use async_graphql::*;

// Guard that requires a specific role
pub struct RoleGuard {
    required_role: Role,
}

impl RoleGuard {
    pub fn new(required_role: Role) -> Self {
        Self { required_role }
    }
}

impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        match ctx.data::<Option<AuthUser>>()? {
            Some(auth_user) if auth_user.role == self.required_role => Ok(()),
            _ => Err(AppError::Unauthorized.into()),
        }
    }
}
