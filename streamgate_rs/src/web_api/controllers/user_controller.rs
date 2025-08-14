use std::{env, sync::Arc};

use crate::{
    AppState, infrastructure::websockets::ws_connection_manager::WebSocketConnectionManager,
};
use actix_web::{Error, HttpRequest, HttpResponse, rt, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;
use log::debug;

pub struct UserController {}

impl UserController {
    pub async fn connect_ws(
        mut req: HttpRequest,
        stream: web::Payload,
        app_state: web::Data<Arc<AppState>>,
    ) -> Result<HttpResponse, Error> {
        let port = u16::from_str_radix(&env::var("SERVER_PORT").unwrap(), 10).unwrap();
        debug!("WS reqest came to port: {}", port);
        let (res, mut stream, mut session, id) =
            WebSocketConnectionManager::establish_connection(&mut req, stream, app_state.clone())
                .await?;

        rt::spawn(async move {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(AggregatedMessage::Text(text)) => {
                        session.text(text).await.unwrap();
                    }

                    Ok(AggregatedMessage::Binary(bin)) => {
                        session.binary(bin).await.unwrap();
                    }

                    Ok(AggregatedMessage::Ping(msg)) => {
                        session.pong(&msg).await.unwrap();
                    }

                    Ok(AggregatedMessage::Close(reason)) => {
                        println!("WebSocket closed: {:?}", reason);
                        let _ = WebSocketConnectionManager::remove_connection_by_driver_id(
                            id,
                            app_state.clone(),
                        )
                        .await;
                        break;
                    }

                    _ => {}
                }
            }
        });

        Ok(res)
    }
}
