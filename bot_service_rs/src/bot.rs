use std::time::Duration;

use crate::{
    gps::gps::GPS,
    models::geoposition::GeoPosition,
    shared::utils::{to_degrees, to_radians},
};

pub struct DriverBot {
    gps: GPS,
}

impl DriverBot {
    pub fn new(initial_position: GeoPosition) -> Self {
        return Self {
            gps: GPS::new(initial_position),
        };
    }

    pub async fn drive(&mut self, destination: GeoPosition) {
        // init position
        self.gps.load_route(destination).await;

        while let Some(next_pos) = self.gps.get_next_position() {
            let curr_pos = self.gps.get_current_position();
            let new_current_pos = self.move_car(curr_pos, &next_pos, 40.0);
            tokio::time::sleep(Duration::from_secs(3)).await;
            self.gps.update_curr_pos(new_current_pos);
        }
    }

    fn move_car(&self, curr_pos: &GeoPosition, next_pos: &GeoPosition, speed: f64) -> GeoPosition {
        const R: f64 = 6371000.0;
        let bearing = to_radians(self.calculate_bearing(curr_pos, next_pos));

        let cur_lat = to_radians(curr_pos.lat);
        let cur_lon = to_radians(curr_pos.lon);

        let distance = speed / R;

        let new_lat = (cur_lat.sin() * distance.cos()
            + cur_lat.cos() * distance.sin() * bearing.cos())
        .asin();

        let new_lon = cur_lon
            + (bearing.sin() * distance.sin() * cur_lat.cos())
                .atan2(distance.cos() - cur_lat.sin() * new_lat.sin());

        GeoPosition::new(to_degrees(new_lat), to_degrees(new_lon))
    }

    fn calculate_bearing(&self, curr_pos: &GeoPosition, next_pos: &GeoPosition) -> f64 {
        let delta_lon = to_radians(next_pos.lon - curr_pos.lon);
        let current_lat = to_radians(curr_pos.lat);
        let next_lat = to_radians(next_pos.lat);

        let x = delta_lon.sin() * next_lat.cos();
        let y = current_lat.cos() * next_lat.sin()
            - current_lat.sin() * next_lat.cos() * delta_lon.cos();

        let bearing = f64::atan2(x, y);
        (to_degrees(bearing) + 360.0) % 360.0
    }
}
