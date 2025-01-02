use uuid::Uuid;

use crate::domain::{
    auth::AuthUser,
    errors::{DomainError, DomainResult},
    models::course::{Course, CourseCreateData, CourseSummary, CourseUpdateData},
    policies::CoursePolicy,
    repositories::CourseRepository,
};

/// CourseService handles all course-related business logic and coordinates between
/// the policy layer and repository layer.
pub struct CourseService {
    repo: CourseRepository,
    policy: CoursePolicy,
}

impl CourseService {
    pub fn new(repo: CourseRepository) -> Self {
        Self {
            repo,
            policy: CoursePolicy::new(),
        }
    }

    /// Create a new course
    pub async fn create_course(
        &self,
        auth_user: &AuthUser,
        data: CourseCreateData,
    ) -> DomainResult<Course> {
        // Users can always create their own courses
        if auth_user.user_id != data.user_id {
            return Err(DomainError::InsufficientPermissions {
                action: "create",
                resource: "course",
            });
        }

        // Domain validation
        Course::validate_new(&data)?;

        self.repo.create(data).await
    }

    /// Get a course by ID if authorized
    pub async fn get_course(
        &self,
        auth_user: Option<&AuthUser>,
        course_id: Uuid,
    ) -> DomainResult<Course> {
        let course = self.repo.get_by_id(course_id).await?;

        if !self.policy.can_view(auth_user, &course) {
            return Err(DomainError::InsufficientPermissions {
                action: "view",
                resource: "course",
            });
        }

        Ok(course)
    }

    /// List courses for a specific user
    pub async fn list_user_courses(
        &self,
        auth_user: Option<&AuthUser>,
        user_id: Uuid,
    ) -> DomainResult<Vec<CourseSummary>> {
        // Check if authorized to view user's courses
        match auth_user {
            Some(auth_user) if auth_user.user_id == user_id || auth_user.role.is_admin() => {
                // Return all courses for owner/admin
                self.repo.get_user_courses(user_id).await
            }
            _ => {
                // Return only published courses for others
                self.repo.get_user_published_courses(user_id).await
            }
        }
    }

    /// Search published courses
    pub async fn search_published_courses(
        &self,
        query: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> DomainResult<Vec<CourseSummary>> {
        self.repo
            .search_published_courses(query, limit, offset)
            .await
    }

    /// Update a course if authorized
    pub async fn update_course(
        &self,
        auth_user: &AuthUser,
        course_id: Uuid,
        data: CourseUpdateData,
    ) -> DomainResult<Course> {
        let course = self.repo.get_by_id(course_id).await?;

        if !self.policy.can_modify(auth_user, &course) {
            return Err(DomainError::InsufficientPermissions {
                action: "update",
                resource: "course",
            });
        }

        // Domain validation
        Course::validate_update(&data)?;

        self.repo.update(course_id, data).await
    }

    /// Publish/unpublish a course
    pub async fn set_course_published(
        &self,
        auth_user: &AuthUser,
        course_id: Uuid,
        is_published: bool,
    ) -> DomainResult<Course> {
        let course = self.repo.get_by_id(course_id).await?;

        if !self.policy.can_modify(auth_user, &course) {
            return Err(DomainError::InsufficientPermissions {
                action: "publish",
                resource: "course",
            });
        }

        self.repo.set_published(course_id, is_published).await
    }

    /// Delete a course if authorized
    pub async fn delete_course(&self, auth_user: &AuthUser, course_id: Uuid) -> DomainResult<()> {
        let course = self.repo.get_by_id(course_id).await?;

        if !self.policy.can_modify(auth_user, &course) {
            return Err(DomainError::InsufficientPermissions {
                action: "delete",
                resource: "course",
            });
        }

        self.repo.delete(course_id).await
    }
}
