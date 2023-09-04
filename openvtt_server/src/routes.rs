use axum::Router;

mod root;
mod upload;

pub const DIST_DIRECTORY: &str = "dist";
pub const UPLOADS_DIRECTORY: &str = "assets/uploads";

pub fn get_router() -> Router {
    Router::new()
        .merge(root::get_router())
        .nest("/upload", upload::get_router())
}
