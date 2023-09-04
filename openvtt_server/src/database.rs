use deadpool_diesel::{
    sqlite::{self, Manager, Pool},
    Runtime,
};
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

mod models;
mod schema;

pub struct DatabasePool {
    pool: Pool,
}

impl DatabasePool {
    pub fn new() -> Self {
        let manager = Manager::new("database.sqlite3", Runtime::Tokio1);
        let pool = sqlite::Pool::builder(manager).max_size(4).build().unwrap();
        DatabasePool { pool }
    }

    pub async fn get_connection<F, R>(&self, f: F) -> Result<R, deadpool_diesel::InteractError>
    where
        F: FnOnce(&mut SqliteConnection) -> R + Send + 'static,
        R: Send + 'static,
    {
        let conn = self.pool.get().await.unwrap();
        conn.interact(f).await
    }

    pub async fn run_migrations(&mut self) {
        self.get_connection(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }
}
