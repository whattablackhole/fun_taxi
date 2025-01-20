use crate::shared::utils::to_radians;
#[derive(Debug)]
pub struct GeoPosition {
    pub lat: f64,
    pub lon: f64
}

impl GeoPosition {
    pub fn new(lat: f64, lon: f64)-> Self {
        return Self {
            lat: lat,
            lon: lon
        }
    }

    pub fn as_radiance(&self) -> Self {
        return Self {
            lat: to_radians( self.lat),
            lon: to_radians( self.lon)
        }
    }
}