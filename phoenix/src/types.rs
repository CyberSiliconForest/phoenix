use crate::state::PhoenixState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub(crate) struct AppState {
    pub phoenix_state: Arc<RwLock<PhoenixState>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct KeepalivePayload {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CommandPayload {
    pub command: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CommandResponse {
    pub result: String,
}
