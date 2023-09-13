use axum::Router;
use listenfd::ListenFd;
use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
    path::Path,
};
use tokio::signal;

use crate::routes::{DIST_DIRECTORY, UPLOADS_DIRECTORY};

mod database;
mod errors;
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
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(database_pool);
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let mut listenfd = ListenFd::from_env();
    let server = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => axum::Server::from_tcp(listener).unwrap(),
        None => axum::Server::bind(&addr),
    };
    server
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
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

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
