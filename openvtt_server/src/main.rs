use axum::Router;
use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
    path::Path,
};

use crate::routes::{DIST_DIRECTORY, UPLOADS_DIRECTORY};

mod database;
mod routes;
mod signaling;

#[tokio::main]
async fn main() {
    setup_logging();
    let mut database_pool = database::DatabasePool::new();
    database_pool.run_migrations().await;
    tokio::spawn(async {
        signaling::signaling_server().await.unwrap();
    });
    // save files to a separate directory to not override files in the current directory
    if let Err(error) =
        tokio::fs::create_dir_all(Path::new(DIST_DIRECTORY).join(UPLOADS_DIRECTORY)).await
    {
        match error.kind() {
            io::ErrorKind::AlreadyExists => {
                tracing::info!("Upload directory already exists");
                Ok(())
            }
            _ => Err(error),
        }
        .expect("could not create upload directory");
    }
    let app = Router::new()
        .merge(routes::get_router())
        .layer(tower_http::trace::TraceLayer::new_for_http());
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn setup_logging() {
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
