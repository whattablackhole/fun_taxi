use std::env;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_ws::Session;
use dotenv::from_filename;

use crate::infrastructure::streaming::driver_data_producer::DriverDataProducer;
use crate::web_api::controllers::driver_controller::DriverController;

pub mod domain;
pub mod infrastructure;
pub mod web_api;

pub struct AppState {
    // tbd
    driver_session: Mutex<Option<Session>>,
    driver_data_producer: Option<DriverDataProducer>,
}

impl AppState {
    fn new() -> Self {
        Self {
            driver_session: Mutex::new(None),
            driver_data_producer: Some(DriverDataProducer::new("localhost:9092")),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_environment();

    let app_state = web::Data::new(Arc::new(AppState::new()));
    let address = env::var("SERVER_IP_ADRESS").unwrap();
    let port =  u16::from_str_radix( &env::var("SERVER_PORT").unwrap(), 10).unwrap();

    println!("Starting server on {} {}", address, port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route(
                "/",
                web::get().to(DriverController::connect_with_bot_driver),
            )
            .route(
                "/hello",
                web::get().to(|| async { HttpResponse::Ok().body("hello") }),
            )
            .route(
                "stop_driver",
                web::post().to(|state: web::Data<Arc<AppState>>| async move {
                    let response = actix_web::HttpResponse::new(StatusCode::OK);
                    let mut session = state.driver_session.lock().unwrap();

                    let curr = session.as_mut().unwrap();

                    curr.text("stop").await.unwrap();

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
