use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
    pub id: Option<i32>,
    pub username: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewPlayer<'a> {
    pub username: &'a str,
}
