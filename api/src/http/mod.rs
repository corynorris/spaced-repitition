use crate::auth::AuthKey;
use crate::config::Config;
use crate::error::Error;
use crate::graphql::{build_schema, SpacedRepetitionSchema};

use async_graphql::http::{GraphiQLPlugin, GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::Html;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod extractor;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Core API state accessible by handler functions
#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
    pub auth_key: AuthKey,
}

async fn graphql_handler(
    Extension(schema): Extension<SpacedRepetitionSchema>,
    Extension(ctx): Extension<ApiContext>,
    auth_user: Option<extractor::RequiredUser>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    // Add context data that will be available to resolvers
    req = req.data(ctx.db).data(ctx.auth_key).data(auth_user);

    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    let plugins = vec![GraphiQLPlugin {
        name: "graphiql-plugin-explorer",
        constructor: "GraphiQLPluginExplorer.explorerPlugin",
        head_assets: Some("<link  rel=\"stylesheet\" href=\"https://unpkg.com/@graphiql/plugin-explorer@3.2.5/dist/style.css\">"),
        body_assets: Some("<script src=\"https://unpkg.com/@graphiql/plugin-explorer@3.2.5/dist/index.umd.js\"></script>"),
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

/// Starts the server with given configuration and database connection
pub async fn start_server(config: Config, db: PgPool) -> anyhow::Result<()> {
    let schema = build_schema(db.clone());

    let app = Router::new()
        .route("/api/graphql", get(graphiql).post(graphql_handler))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(ApiContext {
                    db,
                    auth_key: AuthKey::new(&config.hmac_key),
                }))
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
