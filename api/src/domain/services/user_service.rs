use uuid::Uuid;

use crate::domain::{
    auth::{AuthKey, AuthUser},
    errors::{DomainError, DomainResult},
    models::user::{
        AdminUserRoleUpdateData,
        AdminUserUpdateData,
        Role,
        User,
        UserChangePasswordData,
        UserCreateData,
        UserProfileUpdateData,
        // UserSearchParams,
    },
    policies::UserPolicy,
    repositories::UserRepository,
};

/// UserService handles all user-related business logic and coordinates between
/// the policy layer, repository layer, and authentication.
pub struct UserService {
    repo: UserRepository,
    policy: UserPolicy,
    auth_key: AuthKey,
}

impl UserService {
    pub fn new(repo: UserRepository, auth_key: AuthKey) -> Self {
        Self {
            repo,
            policy: UserPolicy::new(),
            auth_key,
        }
    }

    /// Register a new user
    pub async fn register(&self, data: UserCreateData) -> DomainResult<(User, String)> {
        // Domain validation
        User::validate_new(&data)?;

        // Create user and generate token
        let user = self.repo.create(data).await?;
        let token = self.generate_auth_token(&user)?;

        Ok((user, token))
    }

    /// Authenticate a user and return a token
    pub async fn login(&self, email: String, password: String) -> DomainResult<(User, String)> {
        let user = self.repo.verify_credentials(&email, &password).await?;
        let token = self.generate_auth_token(&user)?;

        Ok((user, token))
    }

    /// Get a user by ID if authorized
    pub async fn get_user(
        &self,
        auth_user: Option<&AuthUser>,
        user_id: Uuid,
    ) -> DomainResult<User> {
        if !self.policy.can_view(auth_user, user_id) {
            return Err(DomainError::InsufficientPermissions {
                action: "view",
                resource: "user",
            });
        }

        self.repo.get_by_id(user_id).await
    }

    /// Search/list users with pagination
    // pub async fn list_users(
    //     &self,
    //     auth_user: &AuthUser,
    //     params: UserSearchParams,
    // ) -> DomainResult<Vec<User>> {
    //     if !self.policy.can_list_users(auth_user) {
    //         return Err(DomainError::InsufficientPermissions {
    //             action: "list",
    //             resource: "users",
    //         });
    //     }

    //     self.repo.search(params).await
    // }

    /// Update authenticated user's own profile
    pub async fn update_own_profile(
        &self,
        auth_user: &AuthUser,
        data: UserProfileUpdateData,
    ) -> DomainResult<User> {
        if !self.policy.can_update_own_profile(auth_user) {
            return Err(DomainError::InsufficientPermissions {
                action: "update",
                resource: "own_profile",
            });
        }

        // Validate update data
        User::validate_profile_update(&data)?;

        self.repo.update_profile(auth_user.user_id, data).await
    }

    /// Admin update of another user's profile
    pub async fn admin_update_user(
        &self,
        auth_user: &AuthUser,
        user_id: Uuid,
        data: AdminUserUpdateData,
    ) -> DomainResult<User> {
        if !self.policy.can_update(auth_user, user_id) {
            return Err(DomainError::InsufficientPermissions {
                action: "update",
                resource: "user",
            });
        }

        // Validate update data
        User::validate_admin_update(&data)?;

        self.repo.admin_update_user(user_id, data).await
    }

    /// Change user's own password
    pub async fn change_password(
        &self,
        auth_user: &AuthUser,
        data: UserChangePasswordData,
    ) -> DomainResult<User> {
        if !self
            .policy
            .can_change_password(auth_user, auth_user.user_id)
        {
            return Err(DomainError::InsufficientPermissions {
                action: "change_password",
                resource: "user",
            });
        }

        // Validate password change data
        User::validate_password_change(&data)?;

        // Verify current password
        self.repo
            .verify_credentials(&auth_user.user_id.to_string(), &data.current_password)
            .await?;

        self.repo.update_password(auth_user.user_id, data).await
    }

    /// Admin change of user role
    pub async fn change_role(
        &self,
        auth_user: &AuthUser,
        user_id: Uuid,
        role: Role,
    ) -> DomainResult<User> {
        if !self.policy.can_change_role(auth_user, user_id) {
            return Err(DomainError::InsufficientPermissions {
                action: "change_role",
                resource: "user",
            });
        }

        let data = AdminUserRoleUpdateData { role };
        self.repo.update_role(user_id, data).await
    }

    /// Delete a user account
    pub async fn delete_user(&self, auth_user: &AuthUser, user_id: Uuid) -> DomainResult<()> {
        if !self.policy.can_delete(auth_user, user_id) {
            return Err(DomainError::InsufficientPermissions {
                action: "delete",
                resource: "user",
            });
        }

        self.repo.delete(user_id).await
    }

    // Helper method to generate auth token
    fn generate_auth_token(&self, user: &User) -> DomainResult<String> {
        let auth_user = AuthUser {
            user_id: user.user_id,
            role: user.role,
        };

        auth_user.create_token(&self.auth_key).map_err(|_| {
            DomainError::BusinessRuleViolation(
                "Failed to generate authentication token".to_string(),
            )
        })
    }
}
