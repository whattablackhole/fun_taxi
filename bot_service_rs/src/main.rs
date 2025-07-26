pub mod bot;
pub mod consts;
pub mod gps;
pub mod models;
pub mod shared;
pub mod api;

use bot::DriverBot;
use models::geoposition::GeoPosition;

use crate::bot::Car;

#[tokio::main]
async fn main() {
    let mut driver = DriverBot::new();
    let (car,sender,receiver) = Car::new(GeoPosition::new(8.681495, 49.41461));
    driver.establish_connection().await;
    driver.start_car(car, receiver, sender).await;

    let (_, _) = tokio::join!(
        driver.connection_handle.unwrap(),
        driver.car_handle.unwrap()
    );
}
