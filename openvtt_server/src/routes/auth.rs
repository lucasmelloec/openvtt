use axum::{extract::State, response::Json, routing::post, Router};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    database::*,
    errors::{auth::AuthError, AppError},
};

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

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn login(
    State(database_pool): State<DatabasePool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>, AppError> {
    use crate::database::schema::players::dsl::*;

    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials.into());
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
        return Err(AuthError::InvalidCredentials.into());
    }
    let claims = Claims {
        sub: player.username.to_owned(),
        exp: 2000000000,
    };
    let secret = "so secretive";
    let key = EncodingKey::from_secret(secret.as_bytes());
    let token = encode(&Header::default(), &claims, &key).unwrap();

    Ok(Json(json!({"access_token": token})))
}

async fn logout() {}
