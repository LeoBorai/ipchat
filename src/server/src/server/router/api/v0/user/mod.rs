pub mod register;

use axum::Router;
use axum::routing::post;

pub fn routes() -> Router {
    Router::new().route("/", post(register::handler))
}
