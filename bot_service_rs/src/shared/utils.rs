use std::f64::consts::PI;

use crate::models::geoposition::GeoPosition;

pub fn to_degrees(radians: f64) -> f64 {
    return radians * 180.0 / PI; 
}

pub fn to_radians(degrees: f64) -> f64 {
    return degrees / 180.0 * PI; 
}

pub fn haversine_distance(pos1: &GeoPosition, pos2: &GeoPosition) -> f64 {
    const R: f64 = 6371000.0; // Earth's radius in meters
    let delta_lat = to_radians(pos2.lat - pos1.lat);
    let delta_lon = to_radians(pos2.lon - pos1.lon);

    let lat1 = to_radians(pos1.lat);
    let lat2 = to_radians(pos2.lat);

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    R * c // Distance in meters
}
// R = 6371000  # Radius of Earth in meters
// lat1, lon1 = pos1.lat, pos1.lng
// lat2, lon2 = pos2.lat, pos2.lng

// lat1_rad, lon1_rad = map(toRadians, [lat1, lon1])
// lat2_rad, lon2_rad = map(toRadians, [lat2, lon2])

// d_lat = lat2_rad - lat1_rad
// d_lon = lon2_rad - lon1_rad

// a = (math.sin(d_lat / 2) ** 2 +
//      math.cos(lat1_rad) * math.cos(lat2_rad) * math.sin(d_lon / 2) ** 2)
// c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

// return R * c