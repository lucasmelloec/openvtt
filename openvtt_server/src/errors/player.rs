use axum::http::StatusCode;

use super::AppError;

pub enum PlayerError {
    UsernameAlreadyExists,
}

impl From<PlayerError> for (StatusCode, &str) {
    fn from(value: PlayerError) -> Self {
        match value {
            PlayerError::UsernameAlreadyExists => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Username already in use")
            }
        }
    }
}

impl From<PlayerError> for AppError {
    fn from(value: PlayerError) -> Self {
        AppError::Player(value)
    }
}
