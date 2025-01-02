/// Re-export domain models for easier access.
pub mod card;
pub mod course;
pub mod lesson;
pub mod user;

/// Publicly expose the models for use in other parts of the application.
pub use course::Course;
pub use lesson::Lesson;
pub use user::Role;
pub use user::User;
