use axum::Extension;
use axum::http::StatusCode;
use tracing::info;

use crate::services::SharedServices;

pub async fn handler(Extension(services): Extension<SharedServices>) -> Result<(), StatusCode> {
    info!(ws_addr=?services.web_socket.addr(), "WebSocket Info.");

    Ok(())
}
