use axum::{
    extract::{Query, State, WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, warn};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

#[derive(Deserialize)]
pub struct ConnectWsQuery {
    #[serde(rename = "user-id")]
    pub user_id: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(query): Query<ConnectWsQuery>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        debug!(
            "[SPAWNED TASK STARTED] Upgrading connection for user: {}",
            query.user_id
        );

        connection_lifecycle(socket, state, query.user_id)
    })
}

async fn connection_lifecycle(mut socket: WebSocket, state: Arc<AppState>, user_id_str: String) {
    debug!("[WS CONNECTION LIFECYCLE STARTED] Parsing uuid");

    let user_id = match Uuid::parse_str(&user_id_str) {
        Ok(id) => id,
        Err(_) => {
            warn!("Invalid UUID provided, closing connection: {}", user_id_str);
            let _ = socket.close().await;
            return;
        }
    };

    let mut receiver = match state.ws_manager.add_connection(user_id, socket).await {
        Ok(rx) => rx,
        Err(e) => {
            error!("Failed to add connection for user {}: {}", user_id, e);
            return;
        }
    };

    while let Some(msg_result) = receiver.next().await {
        match msg_result {
            Ok(msg) => {
                debug!("Received message from {}: {:?}", user_id, msg);
            }
            Err(e) => {
                debug!("WebSocket error for user {}: {}", user_id, e);
                break;
            }
        }
    }

    if let Err(e) = state.ws_manager.remove_connection(user_id).await {
        error!("Failed to clean up connection for user {}: {}", user_id, e);
    }
}
