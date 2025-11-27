use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{
        StatusCode,
        header::{CACHE_CONTROL, CONTENT_TYPE, HeaderMap, HeaderValue, PRAGMA},
    },
    response::{Html, IntoResponse},
    routing::{get, post},
};
use portfolio_server::{
    configuration::ServerSettings,
    media_files::{Track, list_media_files},
};
use std::str::FromStr;
use tokio::fs;
use tokio_util::io::ReaderStream;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let server_settings: ServerSettings = match ServerSettings::load() {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("{}", err);
            panic!("Configuration issue.");
        }
    };

    let app = Router::new()
        .route("/", get(index))
        .with_state(server_settings.clone())
        .route("/list_media", get(list_media))
        .with_state(server_settings.clone())
        .route("/stream_media", post(stream_media))
        .with_state(server_settings.clone())
        .nest_service(
            "/copy",
            ServeDir::new(server_settings.file_locations().frontend_copy()),
        )
        .nest_service(
            "/js",
            ServeDir::new(server_settings.file_locations().frontend_scripts()),
        )
        .nest_service(
            "/css",
            ServeDir::new(server_settings.file_locations().frontend_styles()),
        );

    if let Some(listener_config) = &server_settings.listeners().last() {
        let socket_address =
            std::net::SocketAddrV4::from_str(listener_config.socket_addr()).unwrap();

        let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {
        panic!("Listener socket configuration not found")
    }
}

async fn index(State(config): State<ServerSettings>) -> Html<String> {
    let mut index_page_path: std::path::PathBuf = config.file_locations().frontend_copy();
    index_page_path.push("index.html");
    let index_page = match fs::read_to_string(&index_page_path).await {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error retrieving home page: {}", err);
            "Whoops...".to_string()
        }
    };
    Html(index_page)
}

async fn list_media(State(config): State<ServerSettings>) -> Json<Vec<Track>> {
    let media_list = list_media_files(config.file_locations().media());
    Json(media_list)
}

async fn stream_media(
    State(config): State<ServerSettings>,
    track: Json<Track>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let mut response_body = Body::empty();
    let mut status_code = StatusCode::NOT_FOUND;
    let mut media_file_path = config.file_locations().media().clone();
    media_file_path.push(&track.file_name);
    match tokio::fs::File::open(&media_file_path).await {
        Ok(media_file) => {
            let media_stream = ReaderStream::new(media_file);

            headers.insert(CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));
            headers.insert(
                CACHE_CONTROL,
                HeaderValue::from_static("no-cache, must-revalidate"),
            );
            headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));

            response_body = Body::from_stream(media_stream);
            status_code = StatusCode::OK;
        }
        Err(err) => {
            eprintln!("Error retrieving stream source: {}", err);
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        }
    };
    (status_code, headers, response_body)
}
