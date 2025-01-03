use crate::application::container::ServiceContainer;
use crate::application::graphql::{build_schema, SpacedRepetitionSchema};
use crate::config::Config;
use crate::domain::auth::{AuthKey, AuthUser};

use async_graphql::http::{GraphiQLPlugin, GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use super::container::ApiContext;

async fn graphiql() -> impl IntoResponse {
    let plugins = vec![GraphiQLPlugin {
        name: "graphiql-plugin-explorer",
        constructor: "GraphiQLPluginExplorer.explorerPlugin",
        head_assets: Some("<link  rel=\"stylesheet\" href=\"https://unpkg.com/@graphiql/plugin-explorer@4.0.0-alpha.2/dist/style.css\">"),
        body_assets: Some("<script src=\"https://unpkg.com/@graphiql/plugin-explorer@4.0.0-alpha.2/dist/index.umd.js\"></script>"),
        pre_configs: None,
        props: None,
    }];

    response::Html(
        GraphiQLSource::build()
            .endpoint("/api/graphql")
            .plugins(&plugins)
            .finish(),
    )
}

/// Core API state accessible by handler functions
async fn graphql_handler(
    Extension(schema): Extension<SpacedRepetitionSchema>,
    Extension(ctx): Extension<ApiContext>,
    headers: axum::http::HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let user = extract_user(auth_header, &ctx.auth_key).await;

    let mut request = req.into_inner();
    request = request
        .data(ctx.auth_key.clone())
        .data(ctx.user_service.clone())
        .data(ctx.course_service.clone());

    if let Some(user) = user {
        request = request.data(user);
    }

    schema.execute(request).await.into()
}

async fn extract_user(auth_header: Option<&str>, auth_key: &AuthKey) -> Option<AuthUser> {
    auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|token| AuthUser::from_token(token, auth_key).ok())
}

/// Starts the server with given configuration and
pub async fn start_server(config: Config, db: PgPool) -> anyhow::Result<()> {
    let schema = build_schema(db.clone());

    let container: ServiceContainer = ServiceContainer::new(config.clone(), db)?;

    let app = Router::new()
        .route("/api/graphql", get(graphiql).post(graphql_handler))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(container.create_context()))
                .layer(Extension(schema))
                .layer(TraceLayer::new_for_http()),
        );

    let addr = config.server_addr();
    tracing::info!("Starting server on {}", addr);
    tracing::info!("GraphiQL IDE available at http://{}/api/graphql", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
