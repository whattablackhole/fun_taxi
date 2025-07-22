use serde::Serialize;

#[derive(Serialize)]

pub struct GeoPosition {
    pub lat: String,
    pub lon: String
}
impl GeoPosition {
    pub fn new (lat: String, lon: String) -> Self {
        return Self {
            lat,
            lon
        }
    }
}