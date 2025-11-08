pub mod v0;

pub fn make_api_router() -> axum::Router {
    axum::Router::new().nest("/v0", v0::routes())
}
