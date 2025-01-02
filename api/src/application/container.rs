use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    domain::{
        auth::AuthKey,
        repositories::{CourseRepository, LessonRepository, UserRepository},
        services::{CourseService, LessonService, UserService},
    },
    infrastructure::config::Config,
};

use super::{errors::ApplicationError, graphql::dataloaders::DataLoaders};

#[derive(Clone)]
pub struct ApiContext {
    pub auth_key: AuthKey,
    pub data_loaders: DataLoaders,
    pub course_service: Arc<CourseService>,
    pub user_service: Arc<UserService>,
    pub lesson_service: Arc<LessonService>,
}
#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<UserService>,
    pub lesson_service: Arc<LessonService>,
    pub course_service: Arc<CourseService>,
    pub data_loaders: DataLoaders,
}

pub struct ServiceContainer {
    auth_key: AuthKey,
    pub services: Arc<Services>,
}

impl ServiceContainer {
    pub fn new(config: Config, db: PgPool) -> Result<Self, AppError> {
        let auth_key = AuthKey::new(&config.hmac_key)
            .map_err(|e| ApplicationError::Internal(format!("Failed to create auth key: {}", e)))?;

        let user_repo = Arc::new(UserRepository::new(db.clone()));
        let lesson_repo = Arc::new(LessonRepository::new(db.clone()));
        let course_repo = Arc::new(CourseRepository::new(db.clone()));

        let user_service = Arc::new(UserService::new(user_repo, auth_key.clone()));
        let lesson_service = Arc::new(LessonService::new(lesson_repo));
        let course_service = Arc::new(CourseService::new(course_repo));

        let data_loaders = DataLoaders::new(lesson_service.clone());

        Ok(Self {
            auth_key,
            services: Arc::new(Services {
                user_service,
                lesson_service,
                course_service,
                data_loaders,
            }),
        })
    }

    pub fn create_context(&self) -> ApiContext {
        ApiContext {
            auth_key: self.auth_key.clone(),
            data_loaders: self.services.data_loaders.clone(),
            course_service: self.services.course_service.clone(),
            user_service: self.services.user_service.clone(),
            lesson_service: self.services.lesson_service.clone(),
        }
    }
}
