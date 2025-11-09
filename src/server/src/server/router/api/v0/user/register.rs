use axum::Extension;
use axum::http::StatusCode;
use tracing::{error, info};

use crate::services::SharedServices;

#[utoipa::path(
    get,
    operation_id = "userRegister",
    path = "api/v0/user",
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 500, description = "Internal server error")
    ),
    tag = "user"
)]
pub async fn handler(Extension(services): Extension<SharedServices>) -> Result<(), StatusCode> {
    let user = services.user.create_user("Leo").await.map_err(|err| {
        error!(%err, "Failed to query pending summaries.");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(?user, "Created user successfully.");

    Ok(())
}
