use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::{Html, Redirect},
    routing::get,
    BoxError, Router,
};
use futures::{Stream, TryStreamExt};
use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
    path::Path,
};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower_http::{limit::RequestBodyLimitLayer, services::ServeDir};

mod signaling;

const DIST_DIRECTORY: &str = "dist";
const UPLOADS_DIRECTORY: &str = "assets/uploads";

#[tokio::main]
async fn main() {
    setup_logging();
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
        .nest_service(
            "/",
            ServeDir::new("static").fallback(ServeDir::new(DIST_DIRECTORY)),
        )
        .route("/upload", get(show_form).post(accept_form))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
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

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Upload something!</title>
            </head>
            <body>
                <form action="/upload" method="post" enctype="multipart/form-data">
                    <div>
                        <label>
                            Upload file:
                            <input type="file" name="file" multiple>
                        </label>
                    </div>

                    <div>
                        <input type="submit" value="Upload files">
                    </div>
                </form>
            </body>
        </html>
        "#,
    )
}

async fn accept_form(mut multipart: Multipart) -> Result<Redirect, (StatusCode, String)> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        stream_to_file(&file_name, field).await?;
    }

    Ok(Redirect::to("/upload"))
}

async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(DIST_DIRECTORY)
            .join(UPLOADS_DIRECTORY)
            .join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
