pub mod info;

use axum::Router;
use axum::routing::get;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router {
    Router::new().route("/", get(info::handler))
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct NodeObject {
    /// Installation path of the node
    pub install_path: String,
    /// WebSocket Address
    pub web_socket_addr: String,
}
