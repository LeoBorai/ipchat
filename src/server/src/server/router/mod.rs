pub mod api;
pub mod assets;

use anyhow::Result;
use axum::routing::get;
use axum::{Extension, Router};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::services::SharedServices;

use self::assets::serve_asset;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::server::router::api::v0::node::info::handler,
        crate::server::router::api::v0::user::register::handler,
    ),
    components(
        schemas(crate::server::router::api::v0::node::NodeObject)
    ),
    tags(
        (name = "node", description = "Node Management"),
        (name = "user", description = "User Management")
    ),
    info(
        title = "IPChat API",
        description = "API for IPChat application",
        version = "0.0.0"
    )
)]
struct ApiDoc;

pub async fn make_router(services: SharedServices) -> Result<Router> {
    let router = axum::Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api::make_api_router())
        .fallback_service(get(serve_asset))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(Extension(services));

    Ok(router)
}
