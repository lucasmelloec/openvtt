use axum::Router;
use listenfd::ListenFd;
use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
    path::Path,
};
use tokio::{net::TcpListener, signal};

use crate::routes::UPLOADS_DIRECTORY;

mod routes;

pub async fn start_app() {
    setup_uploads_directory().await;
    let app = Router::new()
        .merge(routes::get_router())
        .layer(tower_http::trace::TraceLayer::new_for_http());
    let mut listenfd = ListenFd::from_env();
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind(addr).await.unwrap(),
    };
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn setup_uploads_directory() {
    // save files to a separate directory to not override files in the current directory
    if let Err(error) = tokio::fs::create_dir_all(Path::new(UPLOADS_DIRECTORY)).await {
        match error.kind() {
            io::ErrorKind::AlreadyExists => {
                tracing::info!("Upload directory already exists");
                Ok(())
            }
            _ => Err(error),
        }
        .expect("could not create upload directory");
    }
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
