use crate::{
    application::graphql::errors::ErrorCode,
    domain::{auth::AuthUser, models::user::Role},
};
use async_graphql::*;
use std::collections::HashSet;

pub struct RoleGuard {
    required_roles: HashSet<Role>,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        let mut roles = HashSet::new();
        roles.insert(role);
        // Automatically add Admin if it's not already the required role
        if role != Role::Admin {
            roles.insert(Role::Admin);
        }
        Self {
            required_roles: roles,
        }
    }

    // For cases where you want to specify multiple roles explicitly
    pub fn with_roles(roles: Vec<Role>) -> Self {
        let mut required_roles: HashSet<_> = roles.into_iter().collect();
        // Always add Admin unless explicitly provided
        if !required_roles.contains(&Role::Admin) {
            required_roles.insert(Role::Admin);
        }
        Self { required_roles }
    }
}

impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        match ctx.data_opt::<AuthUser>() {
            Some(auth_user) if self.required_roles.contains(&auth_user.role) => Ok(()),
            Some(_) => Err(Error::new(format!(
                "You must have one of these roles to perform this action: {:?}",
                self.required_roles
            ))
            .extend_with(|_, e| {
                e.set("code", ErrorCode::Forbidden.as_str());
            })),
            None => Err(
                Error::new("You must be logged in to perform this action").extend_with(|_, e| {
                    e.set("code", ErrorCode::Unauthorized.as_str());
                }),
            ),
        }
    }
}
