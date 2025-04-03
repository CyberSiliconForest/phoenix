mod v1;

use crate::config::Config;
use crate::types::AppState;
use axum::routing::*;

pub async fn get_service(cfg: &Config, app_state: AppState) -> Router<AppState> {
    Router::new().nest("/v1", v1::get_service(cfg, app_state.clone()).await)
}
