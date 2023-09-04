use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use diesel::prelude::*;
use serde::Deserialize;

use crate::database::*;

pub fn get_router() -> Router<DatabasePool> {
    Router::new()
        .route("/", post(create_player))
        .route("/", get(list_players))
}

#[derive(Deserialize)]
struct PlayerPayload {
    username: String,
    password: String,
}

impl PlayerPayload {
    fn hashed_password(&self) -> String {
        bcrypt::hash(&self.password, 14).unwrap()
    }
}

async fn create_player(
    State(database_pool): State<DatabasePool>,
    Json(payload): Json<PlayerPayload>,
) {
    database_pool
        .get_connection(move |conn| {
            let new_player = models::NewPlayer {
                username: &payload.username,
                hashed_password: &payload.hashed_password(),
            };
            diesel::insert_into(schema::players::table)
                .values(&new_player)
                .execute(conn)
                .unwrap()
        })
        .await;
}

async fn list_players(
    State(database_pool): State<DatabasePool>,
) -> Result<Json<Vec<models::Player>>, (StatusCode, String)> {
    let players = database_pool
        .get_connection(|conn| {
            schema::players::table
                .select(models::Player::as_select())
                .load::<models::Player>(conn)
                .unwrap()
        })
        .await;
    Ok(Json(players))
}
