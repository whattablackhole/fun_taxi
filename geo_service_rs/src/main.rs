mod controllers;
mod tools;
mod services;
mod models;

use actix_web::{web, App, HttpServer};
use tools::env_reader::get_env;

struct AppState {
    pub _app_name: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_env = get_env();

    let port = u16::from_str_radix(app_env.get("PORT").unwrap(), 10).unwrap();
    let host = app_env.get("HOST").unwrap().as_str();
    
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                _app_name: String::from("Taxi"),
            }))
            .configure(controllers::navigation::config)
    })
    .bind((host, port))?
    .run()
    .await
}
