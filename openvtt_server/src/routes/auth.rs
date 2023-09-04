use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};
use diesel::prelude::*;
use serde::Deserialize;

use crate::database::*;

pub fn get_router() -> Router<DatabasePool> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

#[derive(Deserialize)]
struct AuthPayload {
    username: String,
    password: String,
}

async fn login(
    State(database_pool): State<DatabasePool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<models::Player>, (StatusCode, String)> {
    use crate::database::schema::players::dsl::*;

    if payload.username.is_empty() || payload.password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_string()));
    }
    let player = database_pool
        .get_connection(|conn| {
            players
                .filter(username.eq(payload.username))
                .select(models::Player::as_select())
                .first(conn)
                .unwrap()
        })
        .await;
    if !bcrypt::verify(&payload.password, &player.hashed_password).unwrap() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Wrong username or password".to_string(),
        ));
    }

    Ok(Json(player))
}

async fn logout() {}
