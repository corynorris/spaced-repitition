use crate::domain::services::LessonService;
use async_graphql::dataloader::*;
use std::sync::Arc;

mod lesson_data_loader;

pub use lesson_data_loader::LessonDataLoader;

/// Helper struct to hold all DataLoader instances
#[derive(Clone)]
pub struct DataLoaders {
    pub lesson: Arc<DataLoader<LessonDataLoader>>,
}

impl DataLoaders {
    pub fn new(lesson_service: Arc<LessonService>) -> Self {
        Self {
            lesson: Arc::new(DataLoader::new(
                LessonDataLoader::new(lesson_service),
                tokio::spawn,
            )),
        }
    }
}
