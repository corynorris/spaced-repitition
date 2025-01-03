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
        match ctx.data_opt::<AuthUser>() {
            Some(auth_user) if auth_user.role == self.required_role => Ok(()),
            Some(_) => Err(async_graphql::Error::new(format!(
                "You must have the `{:?}` role to perform this action",
                self.required_role
            ))),
            _ => Err(async_graphql::Error::new(
                "You must be logged in to perform this action",
            )),
        }
    }
}
