pub mod api;
pub mod assets;

use anyhow::Result;
use axum::routing::get;
use axum::{Extension, Router};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::services::SharedServices;

use self::assets::serve_asset;

pub async fn make_router(services: SharedServices) -> Result<Router> {
    let router = axum::Router::new()
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
