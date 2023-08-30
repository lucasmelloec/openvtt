use axum::Router;
use tower_http::services::ServeDir;

use super::{PUBLIC_DIRECTORY, UPLOADS_DIRECTORY};

pub fn get_router() -> Router {
    Router::new().nest_service(
        "/",
        ServeDir::new(PUBLIC_DIRECTORY).fallback(ServeDir::new(UPLOADS_DIRECTORY)),
    )
}
