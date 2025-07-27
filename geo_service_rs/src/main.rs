mod controllers;
mod tools;
mod services;
mod models;
mod consumers;

use std::env;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use consumers::geoposition::consume_geoposition;
use dotenv::from_filename;
use tools::env_reader::get_env;

struct AppState {
    pub _app_name: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
      load_environment();


    let address = env::var("SERVER_IP_ADRESS").unwrap();
    let port =  u16::from_str_radix( &env::var("SERVER_PORT").unwrap(), 10).unwrap();

    // let port = u16::from_str_radix(app_env.get("PORT").unwrap(), 10).unwrap();
    // let host = app_env.get("HOST").unwrap().as_str();
    
    // consume_geoposition("localhost:9092", "your_group_id", &["driver_geo_position"]).await;

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(AppState {
                _app_name: String::from("Taxi"),
            }))
            .configure(controllers::navigation::config)
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
