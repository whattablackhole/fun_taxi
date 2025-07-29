use crate::models::geoposition::GeoPosition;
use chrono::{DateTime, Utc};
use serde::Serialize;


#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub  struct GeoLocation {
    pub lat: f64,
    pub lon: f64,
}


#[derive(Serialize)]
#[derive(Clone)]
#[serde(into = "u8")]
pub enum GpsMessageType {
    LocationUpdate = 0,
    SomethingElse = 1,
}

impl From<GpsMessageType> for u8 {
    fn from(value: GpsMessageType) -> Self {
        value as u8
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub  struct DriverLocationChangedMessage {
    pub location: GeoLocation,
    pub sent_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DriverMessage {
    pub r#type: GpsMessageType,
    pub driver_id: String,
    pub payload: DriverLocationChangedMessage,
}