use std::sync::Arc;

use axum::Router;

mod auth;
mod players;
mod root;
mod upload;

pub const DIST_DIRECTORY: &str = "dist";
pub const UPLOADS_DIRECTORY: &str = "assets/uploads";

pub fn get_router() -> Router<Arc<crate::database::DatabasePool>> {
    Router::new()
        .merge(root::get_router())
        .merge(auth::get_router())
        .nest("/upload", upload::get_router())
        .nest("/players", players::get_router())
}
