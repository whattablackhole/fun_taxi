mod controllers;
mod tools;
mod services;
mod models;
mod consumers;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use consumers::geoposition::consume_geoposition;
use tools::env_reader::get_env;

struct AppState {
    pub _app_name: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_env = get_env();

    let port = u16::from_str_radix(app_env.get("PORT").unwrap(), 10).unwrap();
    let host = app_env.get("HOST").unwrap().as_str();
    
    // consume_geoposition("localhost:9092", "your_group_id", &["driver_geo_position"]).await;

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(AppState {
                _app_name: String::from("Taxi"),
            }))
            .configure(controllers::navigation::config)
    })
    .bind((host, port))?
    .run()
    .await
}
