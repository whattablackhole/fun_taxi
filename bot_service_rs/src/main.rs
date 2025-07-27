pub mod bot;
pub mod consts;
pub mod gps;
pub mod models;
pub mod shared;
pub mod api;

use bot::DriverBot;
use models::geoposition::GeoPosition;
use std::env;
use crate::bot::Car;
use dotenv::from_filename;

#[tokio::main]
async fn main() {
    load_environment();
    let mut driver = DriverBot::new("1".to_string());
    let (car,sender,receiver) = Car::new(GeoPosition::new(8.681495, 49.41461));
    driver.establish_connection().await;
    driver.start_car(car, receiver, sender).await;

    let (_, _) = tokio::join!(
        driver.connection_handle.unwrap(),
        driver.car_handle.unwrap()
    );
}


fn load_environment() {
    let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
    let filename = format!(".env.{}", env);
    from_filename(&filename).ok();
}