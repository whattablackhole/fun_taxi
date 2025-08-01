use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Deserialize)]
pub struct StartDriverDto {
    pub driver_id: Uuid,
}



#[derive(Deserialize)]
pub struct GetAvailableTripsDto {
    pub driver_id: Uuid,
    pub lat: f64,
    pub lon: f64,
    pub radius: i32
}


#[derive(Serialize)]
pub struct GeoPositionDto {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize)]
pub struct AvailableTripsDto {
    pub id: Uuid,
    pub start: GeoPositionDto,
    pub end: GeoPositionDto,
}