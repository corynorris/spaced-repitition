use crate::domain::auth::AuthUser;
use uuid::Uuid;

/// UserPolicy handles all authorization rules for user-related operations.
/// Each method returns a boolean indicating whether the action is permitted.
///
/// Authorization rules:
/// - Users can view and update their own profile
/// - Admins can view and update any user profile
/// - Only admins can change user roles
/// - Users can delete their own account (except admins)
/// - Admins can delete other users but not themselves
#[derive(Clone, Debug)]
pub struct UserPolicy;

impl UserPolicy {
    pub fn new() -> Self {
        Self
    }

    /// Check if a user can view another user's profile
    ///
    /// Rules:
    /// - Users can view their own profile
    /// - Admins can view any profile
    /// - Unauthenticated users cannot view profiles
    pub fn can_view(&self, auth_user: Option<&AuthUser>, target_user_id: Uuid) -> bool {
        match auth_user {
            Some(auth_user) => auth_user.user_id == target_user_id || auth_user.role.is_admin(),
            None => false,
        }
    }

    /// Check if a user can update another user's profile
    ///
    /// Rules:
    /// - Users can update their own profile
    /// - Admins can update any profile
    pub fn can_update_user_profile(&self, auth_user: &AuthUser, target_user_id: Uuid) -> bool {
        auth_user.role.is_admin() || auth_user.user_id == target_user_id
    }

    /// Check if a user can update their own profile
    /// This is a convenience method that wraps can_update
    pub fn can_update_own_profile(&self, _auth_user: &AuthUser) -> bool {
        // All users can update their own profile
        true
    }

    /// Check if a user can change another user's password
    ///
    /// Rules:
    /// - Users can only change their own password
    /// - Admins cannot change other users' passwords (they should use password reset)
    pub fn can_change_password(&self, auth_user: &AuthUser, target_user_id: Uuid) -> bool {
        auth_user.user_id == target_user_id
    }

    /// Check if a user can change another user's role
    ///
    /// Rules:
    /// - Only admins can change roles
    /// - Admins cannot change their own role (prevents lockout)
    pub fn can_change_role(&self, auth_user: &AuthUser, target_user_id: Uuid) -> bool {
        auth_user.role.is_admin() && auth_user.user_id != target_user_id
    }

    /// Check if a user can delete an account
    ///
    /// Rules:
    /// - Users can delete their own account, unless they are an admin
    /// - Admins can delete other users' accounts
    /// - Admins cannot delete their own account (prevents lockout)
    pub fn can_delete(&self, auth_user: &AuthUser, target_user_id: Uuid) -> bool {
        if auth_user.user_id == target_user_id {
            // Users can delete their own account, unless they're an admin
            !auth_user.role.is_admin()
        } else {
            // Only admins can delete other users
            auth_user.role.is_admin()
        }
    }

    /// Check if a user can list/search users
    ///
    /// Rules:
    /// - Only admins can list/search users
    pub fn can_list_users(&self, auth_user: &AuthUser) -> bool {
        auth_user.role.is_admin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::user::Role;

    fn create_user(id: u128, role: Role) -> AuthUser {
        AuthUser {
            user_id: Uuid::from_u128(id),
            role,
        }
    }

    #[test]
    fn test_can_view() {
        let policy = UserPolicy::new();
        let user_id = Uuid::from_u128(1);
        let admin = create_user(2, Role::Admin);
        let user = create_user(1, Role::User);
        let other_user = create_user(3, Role::User);

        // Users can view their own profile
        assert!(policy.can_view(Some(&user), user_id));

        // Admins can view any profile
        assert!(policy.can_view(Some(&admin), user_id));

        // Users cannot view other users' profiles
        assert!(!policy.can_view(Some(&other_user), user_id));

        // Unauthenticated users cannot view profiles
        assert!(!policy.can_view(None, user_id));
    }

    #[test]
    fn test_can_update() {
        let policy = UserPolicy::new();
        let user_id = Uuid::from_u128(1);
        let admin = create_user(2, Role::Admin);
        let user = create_user(1, Role::User);
        let other_user = create_user(3, Role::User);

        // Users can update their own profile
        assert!(policy.can_update(&user, user.user_id));

        // Admins can update any profile
        assert!(policy.can_update(&admin, user_id));

        // Users cannot update other users' profiles
        assert!(!policy.can_update(&other_user, user_id));
    }

    #[test]
    fn test_can_change_password() {
        let policy = UserPolicy::new();
        let user_id = Uuid::from_u128(1);
        let admin = create_user(2, Role::Admin);
        let user = create_user(1, Role::User);

        // Users can change their own password
        assert!(policy.can_change_password(&user, user.user_id));

        // Even admins can only change their own password
        assert!(!policy.can_change_password(&admin, user_id));
    }

    #[test]
    fn test_can_change_role() {
        let policy = UserPolicy::new();
        let admin = create_user(1, Role::Admin);
        let user = create_user(2, Role::User);
        let other_user_id = Uuid::from_u128(3);

        // Admins can change other users' roles
        assert!(policy.can_change_role(&admin, other_user_id));

        // Regular users cannot change roles
        assert!(!policy.can_change_role(&user, other_user_id));

        // Admins cannot change their own role
        assert!(!policy.can_change_role(&admin, admin.user_id));
    }

    #[test]
    fn test_can_delete() {
        let policy = UserPolicy::new();
        let admin = create_user(1, Role::Admin);
        let user = create_user(2, Role::User);
        let other_user_id = Uuid::from_u128(3);

        // Regular users can delete their own account
        assert!(policy.can_delete(&user, user.user_id));

        // Admins can delete other users' accounts
        assert!(policy.can_delete(&admin, other_user_id));

        // Admins cannot delete their own account
        assert!(!policy.can_delete(&admin, admin.user_id));

        // Regular users cannot delete other users
        assert!(!policy.can_delete(&user, other_user_id));
    }

    #[test]
    fn test_can_list_users() {
        let policy = UserPolicy::new();
        let admin = create_user(1, Role::Admin);
        let user = create_user(2, Role::User);

        // Only admins can list users
        assert!(policy.can_list_users(&admin));
        assert!(!policy.can_list_users(&user));
    }
}
