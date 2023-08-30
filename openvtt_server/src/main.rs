use tracing::info;

#[tokio::main]
async fn main() {
    setup_logging();
    info!("starting openvtt_server");
    openvtt_server::start_app().await;
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
