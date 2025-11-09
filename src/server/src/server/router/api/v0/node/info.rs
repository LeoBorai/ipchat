use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use crate::services::SharedServices;

use super::NodeObject;

#[utoipa::path(
    get,
    operation_id = "nodeInfo",
    path = "api/v0/node",
    responses(
        (status = 200, description = "Info retrieved successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "node"
)]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(NodeObject {
        install_path: services.setup.home_dir().to_string_lossy().to_string(),
        web_socket_addr: services.web_socket.addr().to_string(),
    }))
}
