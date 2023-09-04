use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database::schema;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
    pub id: Option<i32>,
    pub username: String,
    pub hashed_password: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewPlayer<'a> {
    pub username: &'a str,
    pub hashed_password: &'a str,
}
