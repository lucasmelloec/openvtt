use axum::http::StatusCode;

use super::AppError;

pub enum AuthError {
    MissingCredentials,
    InvalidCredentials,
}

impl From<AuthError> for (StatusCode, &str) {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
        }
    }
}

impl From<AuthError> for AppError {
    fn from(value: AuthError) -> Self {
        AppError::Auth(value)
    }
}
