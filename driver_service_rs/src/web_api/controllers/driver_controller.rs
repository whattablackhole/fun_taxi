use std::{str::FromStr, sync::Arc};

use crate::{
    domain::services::trips_finder_service::TripsFinderService,
    web_api::dtos::start_driver_dto::{AvailableTripsDto, GeoPositionDto, GetAvailableTripsDto},
    AppState,
};
use actix_web::{rt, web, Error, HttpRequest, HttpResponse, Responder};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;
use serde_json::Value;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub enum DriverMessageType {
    DriverPositionUpdate,
}

#[derive(serde::Deserialize)]
pub struct DriverServiceMessage {
    pub message_type: DriverMessageType,
    #[serde(flatten)]
    pub inner_message: Value,
}

#[derive(serde::Deserialize)]
pub struct CustomPosition {
    pub driver_id: Uuid,
}

pub struct DriverController {}

impl DriverController {
    pub async fn get_available_trips(
        payload: web::Json<GetAvailableTripsDto>,
        app_state: web::Data<Arc<AppState>>,
    ) -> impl Responder {
        let mut service = TripsFinderService::new(app_state.grpc_channel.clone());
        let trips = service
            .get_available_trips(payload.lat, payload.lon, payload.radius)
            .await
            .unwrap();
        let response: Vec<AvailableTripsDto> = trips
            .iter()
            .map(|t| AvailableTripsDto {
                id: Uuid::from_str(&t.id).unwrap(),
                start: GeoPositionDto {
                    lat: t.start_lat,
                    lon: t.start_lon,
                },
                end: GeoPositionDto {
                    lat: t.end_lat,
                    lon: t.end_lon,
                },
            })
            .collect();
        HttpResponse::Ok().json(response)
    }

    pub async fn connect_with_bot_driver(
        req: HttpRequest,
        stream: web::Payload,
        app_state: web::Data<Arc<AppState>>,
    ) -> Result<HttpResponse, Error> {
        let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

        let mut stream = stream
            .aggregate_continuations()
            .max_continuation_size(2_usize.pow(20));

        {
            let mut session_option = app_state.driver_session.lock().await;

            session_option.replace(session.clone());
        }

        rt::spawn(async move {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(AggregatedMessage::Text(text)) => {
                        println!("Received: {}", text);
                        let message: DriverServiceMessage =
                            serde_json::de::from_str(&text).unwrap();
                        match message.message_type {
                            DriverMessageType::DriverPositionUpdate => {
                                let pos: CustomPosition =
                                    serde_json::from_value(message.inner_message).unwrap();
                                println!("driver id {}", pos.driver_id);
                            }
                        }
                        session.text(text).await.unwrap();
                    }

                    Ok(AggregatedMessage::Binary(bin)) => {
                        session.binary(bin).await.unwrap();
                    }

                    Ok(AggregatedMessage::Ping(msg)) => {
                        session.pong(&msg).await.unwrap();
                    }

                    _ => {}
                }
            }
        });

        Ok(res)
    }
}
