use std::sync::Arc;

use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_ws::{AggregatedMessageStream, Session};
use redis::{AsyncCommands, RedisError};
use uuid::Uuid;

use crate::AppState;

pub struct WebSocketConnectionManager;

impl WebSocketConnectionManager {
    pub async fn establish_connection(
        req: &mut HttpRequest,
        stream: web::Payload,
        app_state: web::Data<Arc<AppState>>,
    ) -> Result<(HttpResponse, AggregatedMessageStream, Session, Uuid), Error> {
        let (res, session, stream) = actix_ws::handle(&req, stream)
            .map_err(|_| actix_web::error::ErrorBadRequest("Invalid x-user-id header format"))?;

        let stream: actix_ws::AggregatedMessageStream = stream
            .aggregate_continuations()
            .max_continuation_size(2_usize.pow(20));
        // let id_header_value = req.headers().get("x-user-id").ok_or_else(|| {
        //     actix_web::error::ErrorBadRequest(
        //         "Headers don't contain required driver id: x-user-id",
        //     )
        // })?;

        // let id_str = id_header_value
        //     .to_str()
        //     .map_err(|_| actix_web::error::ErrorBadRequest("Invalid x-user-id header format"))?;

        // let driver_id = Uuid::parse_str(id_str).map_err(|_| {
        //     actix_web::error::ErrorBadRequest("Can't parse UUID from Header: x-user-id")
        // })?;

        let driver_id = Uuid::new_v4();

        WebSocketConnectionManager::map_driver_to_active_listeners(&driver_id, &app_state)
            .await
            .map_err(|_| {
                actix_web::error::ErrorInternalServerError(
                    "Problem occured during persitisting drivers connection in db",
                )
            })?;

        {
            let mut sessions = app_state.sessions.lock().await;
            sessions.insert(driver_id, session.clone());
        }
        Ok((res, stream, session, driver_id.clone()))
    }

    pub async fn remove_connection_by_driver_id(
        driver_id: Uuid,
        app_state: web::Data<Arc<AppState>>,
    ) -> Result<(), Error> {
        let mut sessions = app_state.sessions.lock().await;

        let result = sessions.remove_entry(&driver_id);

        if let Some((id, session)) = result {
            session.close(None).await.unwrap();

            WebSocketConnectionManager::unmap_driver_from_active_listeners(&id, &app_state)
                .await
                .map_err(|_| {
                    actix_web::error::ErrorInternalServerError(
                        "Problem occured during removal x-user-id's connection from db",
                    )
                })?;
        }
        Ok(())
    }

    async fn map_driver_to_active_listeners(
        driver_id: &Uuid,
        app_state: &web::Data<Arc<AppState>>,
    ) -> Result<(), RedisError> {
        let mut redis = app_state.redis.clone();

        redis
            .set::<String, String, String>(driver_id.to_string(), app_state.hostname.to_string())
            .await?;

        Ok(())
    }

    async fn unmap_driver_from_active_listeners(
        driver_id: &Uuid,
        app_state: &web::Data<Arc<AppState>>,
    ) -> Result<(), RedisError> {
        let mut redis = app_state.redis.clone();

        redis.del::<String, String>(driver_id.to_string()).await?;

        Ok(())
    }
}
