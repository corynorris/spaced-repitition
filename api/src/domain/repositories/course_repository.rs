use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    errors::{DomainError, DomainResult},
    models::course::{Course, CourseCreateData, CourseSummary, CourseUpdateData},
};

pub struct CourseRepository {
    pool: PgPool,
}

impl CourseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get a course by its ID
    pub async fn get_by_id(&self, course_id: Uuid) -> DomainResult<Course> {
        sqlx::query_as!(
            Course,
            r#"
            SELECT 
                course_id,
                user_id,
                title,
                description,
                is_published,
                created_at,
                updated_at
            FROM course
            WHERE course_id = $1
            "#,
            course_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?
        .ok_or_else(|| DomainError::EntityNotFound {
            entity: "course",
            key: "id",
            value: course_id.to_string(),
        })
    }

    /// Create a new course
    pub async fn create(&self, data: CourseCreateData) -> DomainResult<Course> {
        match sqlx::query_as!(
            Course,
            r#"
            INSERT INTO course (user_id, title, description)
            VALUES ($1, $2, $3)
            RETURNING 
                course_id,
                user_id,
                title,
                description,
                is_published,
                created_at,
                updated_at
            "#,
            data.user_id,
            data.title,
            data.description,
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(course) => Ok(course),
            Err(e) => {
                if let Some(db_error) = e.as_database_error() {
                    if let Some("course_user_id_title_key") = db_error.constraint() {
                        return Err(DomainError::UniqueConstraintViolation {
                            field: "title",
                            value: format!("for user {}", data.user_id),
                        });
                    }
                }
                Err(DomainError::Database(e.to_string()))
            }
        }
    }

    /// Update a course's details
    pub async fn update(&self, course_id: Uuid, data: CourseUpdateData) -> DomainResult<Course> {
        match sqlx::query_as!(
            Course,
            r#"
            UPDATE course
            SET 
                title = COALESCE($1, title),
                description = COALESCE($2, description),
                updated_at = NOW()
            WHERE course_id = $3
            RETURNING 
                course_id,
                user_id,
                title,
                description,
                is_published,
                created_at,
                updated_at
            "#,
            data.title,
            data.description,
            course_id,
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(course) => Ok(course),
            Err(e) => {
                if let Some(db_error) = e.as_database_error() {
                    if let Some("course_user_id_title_key") = db_error.constraint() {
                        return Err(DomainError::UniqueConstraintViolation {
                            field: "title",
                            value: "already exists for this user".to_string(),
                        });
                    }
                }
                Err(DomainError::Database(e.to_string()))
            }
        }
    }

    /// Set a course's published status
    pub async fn set_published(&self, course_id: Uuid, is_published: bool) -> DomainResult<Course> {
        sqlx::query_as!(
            Course,
            r#"
            UPDATE course
            SET 
                is_published = $1,
                updated_at = NOW()
            WHERE course_id = $2
            RETURNING 
                course_id,
                user_id,
                title,
                description,
                is_published,
                created_at,
                updated_at
            "#,
            is_published,
            course_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))
    }

    /// Delete a course
    pub async fn delete(&self, course_id: Uuid) -> DomainResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM course
            WHERE course_id = $1
            "#,
            course_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::EntityNotFound {
                entity: "course",
                key: "id",
                value: course_id.to_string(),
            });
        }

        Ok(())
    }

    /// Get all courses for a user (including unpublished)
    pub async fn get_user_courses(&self, user_id: Uuid) -> DomainResult<Vec<CourseSummary>> {
        sqlx::query!(
            r#"
        SELECT 
            course_id as "course_id!: Uuid",
            title as "title!",
            user_id as "user_id!: Uuid",
            description,
            is_published as "is_published!",
            lesson_count,
            total_cards,
            created_at as "created_at!",
            updated_at
        FROM course_summary
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
            user_id
        )
        .map(|row| CourseSummary {
            course_id: row.course_id,
            title: row.title,
            user_id: row.user_id,
            description: row.description,
            is_published: row.is_published,
            lesson_count: row.lesson_count.unwrap_or(0),
            total_cards: row.total_cards.unwrap_or(0),
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))
    }

    /// Get only published courses for a user
    pub async fn get_user_published_courses(
        &self,
        user_id: Uuid,
    ) -> DomainResult<Vec<CourseSummary>> {
        sqlx::query!(
            r#"
        SELECT 
            course_id as "course_id!: Uuid",
            title as "title!",
            user_id as "user_id!: Uuid",
            description,
            is_published as "is_published!",
            lesson_count,
            total_cards,
            created_at as "created_at!",
            updated_at
        FROM course_summary
        WHERE user_id = $1 AND is_published = true
        ORDER BY created_at DESC
        "#,
            user_id
        )
        .map(|row| CourseSummary {
            course_id: row.course_id,
            title: row.title,
            user_id: row.user_id,
            description: row.description,
            is_published: row.is_published,
            lesson_count: row.lesson_count.unwrap_or(0),
            total_cards: row.total_cards.unwrap_or(0),
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))
    }

    /// Search published courses with optional filtering
    pub async fn search_published_courses(
        &self,
        query: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> DomainResult<Vec<CourseSummary>> {
        let limit = limit.unwrap_or(10).max(1).min(100);
        let offset = offset.unwrap_or(0).max(0);

        let query = query.unwrap_or_default();
        let search_query = format!("%{}%", query.to_lowercase());

        sqlx::query!(
            r#"
        SELECT 
            course_id as "course_id!: Uuid",
            title as "title!",
            user_id as "user_id!: Uuid",
            description,
            is_published as "is_published!",
            lesson_count,
            total_cards,
            created_at as "created_at!",
            updated_at
        FROM course_summary
        WHERE is_published = true
            AND (
                LOWER(title) LIKE $1 
                OR LOWER(description) LIKE $1
            )
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
            search_query,
            limit as i64,
            offset as i64
        )
        .map(|row| CourseSummary {
            course_id: row.course_id,
            title: row.title,
            user_id: row.user_id,
            description: row.description,
            is_published: row.is_published,
            lesson_count: row.lesson_count.unwrap_or(0),
            total_cards: row.total_cards.unwrap_or(0),
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))
    }
}
