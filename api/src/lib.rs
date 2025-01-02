/// Application layer: GraphQL API and HTTP server implementation
pub mod application {
    pub mod container;
    pub mod graphql;
    pub mod http;
}

/// Domain layer: Core business logic, models, policies, repositories, and services
pub mod domain {
    pub mod auth;
    pub mod errors;
    pub mod models;
    pub mod policies;
    pub mod repositories;
    pub mod services;
}

pub mod config;
