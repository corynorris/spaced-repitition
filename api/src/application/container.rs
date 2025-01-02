use std::sync::Arc;

use sqlx::PgPool;
use tower::Service;

use crate::{
    config::Config,
    domain::{
        auth::AuthKey,
        repositories::{CourseRepository, UserRepository},
        services::{CourseService, UserService},
    },
};

#[derive(Clone)]
pub struct ApiContext {
    pub auth_key: AuthKey,
    pub user_service: Arc<UserService>,
    pub course_service: Arc<CourseService>,
}
#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<UserService>,
    pub course_service: Arc<CourseService>,
}

pub struct ServiceContainer {
    auth_key: AuthKey,
    pub services: Arc<Services>,
}

impl ServiceContainer {
    pub fn new(config: Config, db: PgPool) -> Result<ServiceContainer, anyhow::Error> {
        let auth_key = AuthKey::new(&config.hmac_key)?;

        let user_repo = Arc::new(UserRepository::new(db.clone()));
        let course_repo = Arc::new(CourseRepository::new(db.clone()));

        let user_service = Arc::new(UserService::new(user_repo, auth_key.clone()));
        let course_service = Arc::new(CourseService::new(course_repo));

        Ok(Self {
            auth_key,
            services: Arc::new(Services {
                user_service,
                course_service,
            }),
        })
    }

    pub fn create_context(&self) -> ApiContext {
        ApiContext {
            auth_key: self.auth_key.clone(),
            course_service: self.services.course_service.clone(),
            user_service: self.services.user_service.clone(),
        }
    }
}
