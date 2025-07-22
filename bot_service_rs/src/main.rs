pub mod bot;
pub mod consts;
pub mod gps;
pub mod models;
pub mod shared;

use std::sync::Arc;

use bot::DriverBot;
use models::geoposition::GeoPosition;

#[tokio::main]
async fn main() {
    let mut driver = DriverBot::new();
    driver.establish_connection().await;
    driver.spawn_car(GeoPosition::new(8.681495, 49.41461)).await;

    let (_, _) = tokio::join!(
        driver.connection_handle.unwrap(),
        driver.car_handle.unwrap()
    );

    // let mut driver1 = DriverBot::new();
    // driver1.establish_connection().await;
    // driver1.spawn_car(GeoPosition::new(8.681495, 49.41461)).await;

    // let mut driver2 = DriverBot::new();
    // driver2.establish_connection().await;
    // driver2.spawn_car(GeoPosition::new(8.681495, 49.41461)).await;
}
