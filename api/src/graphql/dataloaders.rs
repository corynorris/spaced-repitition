use std::sync::Arc;

use async_graphql::dataloader::*;
use sqlx::PgPool;

mod lesson_loader;

pub use lesson_loader::LessonLoader;

/// Helper struct to hold all DataLoader instances
#[derive(Clone)]
pub struct Loaders {
    pub lesson: Arc<DataLoader<LessonLoader>>,
}

impl Loaders {
    pub fn new(pool: PgPool) -> Self {
        Self {
            lesson: Arc::new(DataLoader::new(
                LessonLoader::new(pool.into()),
                tokio::spawn,
            )),
        }
    }
}
