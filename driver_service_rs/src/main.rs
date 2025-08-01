use std::env;
use std::sync::Arc;

use crate::domain::services::trips_finder_service::TripsFinderService;
use crate::infrastructure::streaming::driver_data_producer::DriverDataProducer;
use crate::web_api::controllers::driver_controller::DriverController;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_ws::Session;
use dotenv::from_filename;
use tokio::sync::Mutex;

pub mod domain;
pub mod infrastructure;
pub mod web_api;

pub struct AppState {
    // tbd
    driver_session: Mutex<Option<Session>>,
    // driver_data_producer: Option<DriverDataProducer>,
    trips_finder: Mutex<Option<TripsFinderService>>,
}

impl AppState {
    async fn new() -> Self {
        Self {
            driver_session: Mutex::new(None),
            // driver_data_producer: Some(DriverDataProducer::new("localhost:9092")),
            trips_finder: Mutex::new(Some(
                TripsFinderService::new(
                    env::var("TRIPS_COORDINATOR_IP_ADDRESS")
                        .unwrap()
                        .to_string(),
                )
                .await
                .unwrap(),
            )),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_environment();

    let app_state = web::Data::new(Arc::new(AppState::new().await));
    let address = env::var("SERVER_IP_ADDRESS").unwrap();
    let port = u16::from_str_radix(&env::var("SERVER_PORT").unwrap(), 10).unwrap();

    println!("Starting server on {} {}", address, port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route(
                "/",
                web::get().to(DriverController::connect_with_bot_driver),
            )
            .route(
                "/get_available_trips",
                web::post().to(DriverController::get_available_trips),
            )
            .route(
                "stop_driver",
                web::post().to(|state: web::Data<Arc<AppState>>| async move {
                    let response = actix_web::HttpResponse::new(StatusCode::OK);
                    let mut session = state.driver_session.lock().await;

                    let cur = session.as_mut().unwrap();
                    cur.text("stop").await.unwrap();

                    response
                }),
            )
        // .route(
        //     "start_driver",
        //     web::post().to(|state| async move {

        //     })
        // )
    })
    .bind((address, port))?
    .run()
    .await
}

fn load_environment() {
    let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
    let filename = format!(".env.{}", env);
    from_filename(&filename).ok();
}
