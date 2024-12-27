use async_graphql::*;
use async_trait::async_trait;

use crate::auth::AuthUser;
use crate::error::Error;

// Guard that requires authentication
pub struct AuthGuard;

#[async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        match ctx.data::<Option<AuthUser>>()? {
            Some(_) => Ok(()),
            None => Err(Error::Unauthorized.into()),
        }
    }
}

// Guard that requires admin role
pub struct AdminGuard;

#[async_trait]
impl Guard for AdminGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        match ctx.data::<Option<AuthUser>>()? {
            Some(auth_user) if auth_user.is_admin => Ok(()),
            Some(_) => Err(Error::Forbidden.into()),
            None => Err(Error::Unauthorized.into()),
        }
    }
}

// Helper functions to use in resolvers
pub fn require_auth<T>(_obj: &T) -> Guard<AuthGuard> {
    Guard::new(AuthGuard)
}

pub fn require_admin<T>(_obj: &T) -> Guard<AdminGuard> {
    Guard::new(AdminGuard)
}
