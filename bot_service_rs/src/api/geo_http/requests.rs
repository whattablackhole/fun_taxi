use serde::Serialize;

use crate::models::geoposition::GeoPosition;

#[derive(Serialize)]
pub struct NavigationInfo {
    start: String,
    end: String,
    profile: String
}

impl NavigationInfo {
    pub fn new (start_position: &GeoPosition, end_position: &GeoPosition, profile: &str) -> Self {
        return Self {
            profile: profile.to_string(),
            start: format!("{},{}", start_position.lon, start_position.lat),
            end: format!("{},{}",  end_position.lon, end_position.lat)
        }
    }
}