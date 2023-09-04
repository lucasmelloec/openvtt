use axum::Router;
use tower_http::services::ServeDir;

use super::DIST_DIRECTORY;

pub fn get_router() -> Router<crate::database::DatabasePool> {
    Router::new().nest_service(
        "/",
        ServeDir::new("static").fallback(ServeDir::new(DIST_DIRECTORY)),
    )
}
