pub mod gps;
pub mod shared;
pub mod bot;
pub mod models;
pub mod consts;

use models::geoposition::GeoPosition;

#[tokio::main]
async fn main() {
    let mut driver=  bot::DriverBot::new(GeoPosition::new(8.681495, 49.41461));
    driver.drive(GeoPosition::new(8.687872, 49.420318)).await;
    let mut driver1=  bot::DriverBot::new(GeoPosition::new(8.681495, 49.41461));
    driver1.drive(GeoPosition::new(8.687872, 49.420318)).await;
}
