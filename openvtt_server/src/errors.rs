use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use self::{auth::AuthError, player::PlayerError};

pub mod auth;
pub mod player;

pub enum AppError {
    Auth(AuthError),
    Player(PlayerError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Auth(auth_error) => auth_error.into(),
            AppError::Player(player_error) => player_error.into(),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
