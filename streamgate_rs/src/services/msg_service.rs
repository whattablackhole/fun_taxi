use std::{env, sync::Arc};

use futures_util::SinkExt;
use log::{debug, info};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::{
    infrastructure::websockets::ws_connection_manager::WebSocketConnectionManager,
    services::streamgate::{
        StreamGateMessage, StreamGateMessageReply,
        stream_gate_message_service_server::StreamGateMessageService,
    },
};

pub struct StreamGateMessageServiceImpl {
    pub ws_manager: Arc<WebSocketConnectionManager>,
}

#[tonic::async_trait]
impl StreamGateMessageService for StreamGateMessageServiceImpl {
    async fn send_message(
        &self,
        request: Request<StreamGateMessage>,
    ) -> Result<Response<StreamGateMessageReply>, tonic::Status> {
        info!("Processing StreamGateMessage send_message method");
        let msg = request.into_inner();
        let id = Uuid::parse_str(&msg.user_id).map_err(|e| {
            tonic::Status::invalid_argument(format!("User id cannot be parsed as UUID, {:}", e))
        })?;
        let port = u16::from_str_radix(&env::var("SERVER_PORT").unwrap(), 10).unwrap();
        debug!("user id and current port is: {} {}", id, port);
        let session_m = {
            let mut sessions = self.ws_manager.sessions.lock().await;
            debug!("sessions len: {}", sessions.len());
            let session = sessions
                .get_mut(&id)
                .ok_or_else(|| tonic::Status::not_found("Session is not found"))?;
            session.clone()
        };

        {
            session_m
                .lock()
                .await
                .send(axum::extract::ws::Message::Binary(msg.payload.into()))
                .await
                .map_err(|e| {
                    tonic::Status::invalid_argument(format!(
                        "Error occured while sending message to user, {:}",
                        e
                    ))
                })?;
        }

        return Ok(Response::new(StreamGateMessageReply {
            ok: true,
            user_id: id.to_string(),
        }));

    }
}
