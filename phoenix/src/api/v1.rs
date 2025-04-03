use crate::config::Config;
use crate::types::{AppState, CommandPayload, CommandResponse, KeepalivePayload};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::*;
use axum_serde::Sonic;

pub async fn heartbeat() -> Result<String, StatusCode> {
    Ok("Phoenix is up and running.".to_owned())
}

pub async fn keepalive(
    State(state): State<AppState>,
    Sonic(payload): Sonic<KeepalivePayload>,
) -> Result<Sonic<CommandResponse>, StatusCode> {
    let phoenix_state = state.phoenix_state.write().await;
    let response = CommandResponse {
        result: "ok".to_owned(),
    };

    Ok(Sonic(response))
}

pub async fn command(
    State(state): State<AppState>,
    Sonic(payload): Sonic<CommandPayload>,
) -> Result<Sonic<CommandResponse>, StatusCode> {
    let response = CommandResponse {
        result: "ok".to_owned(),
    };

    Ok(Sonic(response))
}

pub async fn get_service(cfg: &Config, app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/get", get(heartbeat))
        .route("/keepalive", post(keepalive))
        .route("/command", post(command))
}
