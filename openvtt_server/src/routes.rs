use axum::Router;

mod root;
mod upload;

pub const UPLOADS_DIRECTORY: &str = "uploads";
pub const PUBLIC_DIRECTORY: &str = "public";

pub fn get_router() -> Router {
    Router::new()
        .merge(root::get_router())
        .nest("/upload", upload::get_router())
}
