use axum::extract::ws::{Message, WebSocket};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use log::{debug, error};
use redis::{AsyncCommands, RedisError, aio::MultiplexedConnection};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

type WsTx = SplitSink<WebSocket, Message>;

#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    #[error("Redis command failed: {0}")]
    Redis(#[from] RedisError),
    #[error("Failed to send message to client")]
    SendError,
}

pub struct WebSocketConnectionManager {
    pub sessions: Mutex<HashMap<Uuid, Arc<Mutex<WsTx>>>>,
    pub redis: MultiplexedConnection,
    pub hostname: String,
}

impl WebSocketConnectionManager {
    pub async fn add_connection(
        &self,
        user_id: Uuid,
        socket: WebSocket,
    ) -> Result<SplitStream<WebSocket>, ManagerError> {
        self.map_user_to_active_listeners(&user_id).await?;

        let (sender, receiver) = socket.split();

        {
            let mut sessions = self.sessions.lock().await;
            sessions.insert(user_id, Arc::new(Mutex::new(sender)));
        }

        debug!("Added WebSocket connection for user: {}", user_id);

        Ok(receiver)
    }

    pub async fn remove_connection(&self, user_id: Uuid) -> Result<(), ManagerError> {
        let removed_session = {
            let mut sessions = self.sessions.lock().await;
            sessions.remove(&user_id)
        };

        if let Some(sender) = removed_session {
            if let Err(e) = sender.lock().await.close().await {
                debug!("Error closing socket for user {}: {}", user_id, e);
            }
        }

        self.unmap_user_from_active_listeners(&user_id).await?;

        debug!("Removed WebSocket connection for user: {}", user_id);
        Ok(())
    }

    async fn map_user_to_active_listeners(&self, user_id: &Uuid) -> Result<(), RedisError> {
        self.redis
            .clone()
            .set(user_id.to_string(), self.hostname.clone())
            .await
    }

    async fn unmap_user_from_active_listeners(&self, user_id: &Uuid) -> Result<(), RedisError> {
        self.redis.clone().del(user_id.to_string()).await
    }
}
