use crate::config::Config;
use crate::state::PhoenixState;
use crate::types::AppState;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use std::sync::Arc;
use templates::statics::StaticFile;
use tokio::sync::RwLock;

#[macro_use]
mod axum_ructe;

mod api;
mod config;
mod state;
mod types;

#[tokio::main]
async fn main() {
    let config = Config {};
    let app_state = AppState {
        phoenix_state: Arc::new(RwLock::new(PhoenixState::new())),
    };

    let app = Router::new()
        .route("/", get(index_page))
        .route("/static/{filename}", get(static_files))
        .nest("/api", api::get_service(&config, app_state.clone()).await)
        .with_state(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Handler for static files.
/// Create a response from the file data with a correct content type
/// and a far expires header (or a 404 if the file does not exist).
async fn static_files(Path(filename): Path<String>) -> Response {
    match StaticFile::get(&filename) {
        Some(data) => {
            (
                [
                    (header::CONTENT_TYPE, data.mime.as_ref()),
                    (
                        header::CACHE_CONTROL,
                        // max age is 180 days (given in seconds)
                        "public, max_age=15552000, immutable",
                    ),
                ],
                data.content,
            )
                .into_response()
        }
        None => handler_404().await.into_response(),
    }
}

/// Home page handler; just render a template with some arguments.
async fn index_page(State(state): State<AppState>) -> impl IntoResponse {
    let phoenix_state = state.phoenix_state.read().await;
    let (phase, expires) = phoenix_state.get_state();
    let expire_ts = match expires {
        Some(expires) => expires.to_string(),
        None => "Not set".to_string(),
    };
    render!(templates::index_html, &(phase.into(), &expire_ts))
}

async fn handler_404() -> impl IntoResponse {
    error_response(
        StatusCode::NOT_FOUND,
        "The resource you requested can't be found.",
    )
}

fn error_response(status_code: StatusCode, message: &str) -> impl IntoResponse + '_ {
    (
        status_code,
        render!(templates::error_html, status_code, message),
    )
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// Include template files
include!(concat!(env!("OUT_DIR"), "/templates.rs"));
