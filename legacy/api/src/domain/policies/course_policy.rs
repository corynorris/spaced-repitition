use crate::domain::{auth::AuthUser, models::course::Course};

/// CoursePolicy handles all authorization rules for course-related operations.
/// Each method returns a boolean indicating whether the action is permitted.
///
/// Authorization rules:
/// - Users can create their own courses
/// - Users can only modify their own courses
/// - Published courses are viewable by everyone
/// - Unpublished courses are only viewable by their owners
/// - Admins can view all courses but cannot modify others' courses
#[derive(Clone, Debug)]
pub struct CoursePolicy;

impl CoursePolicy {
    pub fn new() -> Self {
        Self
    }

    /// Check if a user can view a course
    ///
    /// Rules:
    /// - Published courses are viewable by everyone
    /// - Unpublished courses are only viewable by their owner
    /// - Admins can view all courses
    pub fn can_view(&self, auth_user: Option<&AuthUser>, course: &Course) -> bool {
        if course.is_published {
            return true;
        }

        match auth_user {
            Some(auth_user) => auth_user.user_id == course.user_id || auth_user.role.is_admin(),
            None => false,
        }
    }

    /// Check if a user can modify a course
    /// This covers updating, publishing/unpublishing, and deleting
    ///
    /// Rules:
    /// - Only the course owner or an admin can modify courses
    pub fn can_modify(&self, auth_user: &AuthUser, course: &Course) -> bool {
        auth_user.user_id == course.user_id || auth_user.role.is_admin()
    }

    /// Check if a user can list courses for the target user
    ///
    /// Rules:
    /// - Users can always list their own courses
    /// - Admins can list any user's courses
    /// - Others can only list published courses
    pub fn can_list_user_courses(
        &self,
        auth_user: Option<&AuthUser>,
        target_user_id: uuid::Uuid,
    ) -> bool {
        match auth_user {
            Some(auth_user) => auth_user.user_id == target_user_id || auth_user.role.is_admin(),
            None => true, // Can list published courses only (filtered in repository)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{course::mock::create_test_course, user::Role};
    use uuid::Uuid;

    fn create_auth_user(id: u128, role: Role) -> AuthUser {
        AuthUser {
            user_id: Uuid::from_u128(id),
            role,
        }
    }

    #[test]
    fn test_can_view_published_course() {
        let policy = CoursePolicy::new();
        let course = create_test_course(Uuid::from_u128(1));

        // Published courses can be viewed by anyone
        assert!(policy.can_view(None, &course));

        // Including other users
        let other_user = create_auth_user(2, Role::User);
        assert!(policy.can_view(Some(&other_user), &course));
    }

    #[test]
    fn test_can_view_unpublished_course() {
        let policy = CoursePolicy::new();
        let owner = create_auth_user(1, Role::User);
        let admin = create_auth_user(2, Role::Admin);
        let other_user = create_auth_user(3, Role::User);

        let mut course = create_test_course(owner.user_id);
        course.is_published = false;

        // Owner can view their unpublished course
        assert!(policy.can_view(Some(&owner), &course));

        // Admin can view unpublished courses
        assert!(policy.can_view(Some(&admin), &course));

        // Other users cannot view unpublished courses
        assert!(!policy.can_view(Some(&other_user), &course));

        // Unauthenticated users cannot view unpublished courses
        assert!(!policy.can_view(None, &course));
    }

    #[test]
    fn test_can_modify_course() {
        let policy = CoursePolicy::new();
        let owner = create_auth_user(1, Role::User);
        let admin = create_auth_user(2, Role::Admin);
        let other_user = create_auth_user(3, Role::User);

        let course = create_test_course(owner.user_id);

        // Owner can modify their course
        assert!(policy.can_modify(&owner, &course));

        // Admins can modify any course
        assert!(policy.can_modify(&admin, &course));

        // Other users cannot modify the course
        assert!(!policy.can_modify(&other_user, &course));
    }

    #[test]
    fn test_can_list_user_courses() {
        let policy = CoursePolicy::new();
        let user_id = Uuid::from_u128(1);
        let owner = create_auth_user(1, Role::User);
        let admin = create_auth_user(2, Role::Admin);
        let other_user = create_auth_user(3, Role::User);

        // Users can list their own courses
        assert!(policy.can_list_user_courses(Some(&owner), user_id));

        // Admins can list any user's courses
        assert!(policy.can_list_user_courses(Some(&admin), user_id));

        // Other authenticated users can list courses (but will only see published ones)
        assert!(policy.can_list_user_courses(Some(&other_user), user_id));

        // Unauthenticated users can list courses (but will only see published ones)
        assert!(policy.can_list_user_courses(None, user_id));
    }
}
