pub mod info;

use axum::Router;
use axum::routing::get;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router {
    Router::new().route("/", get(info::handler))
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NodeObject {
    /// Installation path of the node
    pub install_path: String,
    /// Client's IP Address
    pub client_ip: String,
    /// Node's Local IP Address
    pub local_ip: String,
    /// WebSocket Address
    pub web_socket_addr: String,
}
