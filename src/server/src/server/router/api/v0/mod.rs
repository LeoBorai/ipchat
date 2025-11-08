pub mod node;
pub mod user;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .nest("/node", node::routes())
        .nest("/user", user::routes())
}
