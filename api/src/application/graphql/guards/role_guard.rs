use crate::domain::{auth::AuthUser, models::user::Role};
use async_graphql::*;

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
        match ctx.data_opt::<Option<AuthUser>>() {
            Some(Some(auth_user)) if auth_user.role == self.required_role => Ok(()),
            _ => Err(async_graphql::Error::new("Unauthorized")),
        }
    }
}
