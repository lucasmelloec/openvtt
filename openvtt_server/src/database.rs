use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn new() -> Self {
        Database {
            connection: SqliteConnection::establish("database.sqlite3").unwrap(),
        }
    }

    pub fn run_migrations(&mut self) {
        self.connection.run_pending_migrations(MIGRATIONS).unwrap();
    }
}
