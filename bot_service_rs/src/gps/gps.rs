use crate::consts::GEO_SERVICE_URL;
use crate::models::response::{NavigationFeature, NavigationFeatureSegment, NavigationFeatureStep};
use crate::models::{
    geoposition::GeoPosition, requests::NavigationInfo, response::NavigationCollection,
};
use crate::shared::utils::haversine_distance;

pub struct GPS {
    current_position: GeoPosition,
    _current_bearing: f64,
    navigation_route: Option<NavigationCollection>,
    current_feature_cursor: usize,
    current_segment_cursor: usize,
    current_step_cursor: usize,
    current_geometry_cursor: usize,
    threshold: f64,
}

impl GPS {
    pub fn new(initial_position: GeoPosition) -> Self {
        return Self {
            _current_bearing: 0.0,
            current_position: initial_position,
            navigation_route: None,
            current_feature_cursor: 0,
            current_segment_cursor: 0,
            current_step_cursor: 0,
            current_geometry_cursor: 0,
            threshold: 25.0,
        };
    }

    // TODO:
    // add reset logic

    pub fn update_curr_pos(&mut self, pos: GeoPosition) {
        self.current_position.lat = pos.lat;
        self.current_position.lon = pos.lon;
        //  TODO: stream to outsource
    }

    // NOTE: easier to just use geometry without tracking other cursors
    pub fn get_next_position(&mut self) -> Option<GeoPosition> {
        while let Some(_) = self.get_feature() {
            let current_segment = self.get_current_segment();
            if let Some(_) = current_segment {
                let current_step = self.get_step();
                if let Some(step) = current_step {
                    let end_geometry = step.way_points[1];

                    let geometry = self.get_geometry_cordinates(self.current_geometry_cursor);

                    if let Some(lat_lon) = geometry {
                        let lat = lat_lon[0];
                        let lon = lat_lon[1];
                        let next_pos: GeoPosition = GeoPosition::new(lat, lon);
                        let distance = haversine_distance(&self.current_position, &next_pos);

                        if distance <= self.threshold {
                            if end_geometry > self.current_geometry_cursor {
                                self.current_geometry_cursor += 1;
                            } else {
                                self.current_step_cursor += 1;
                            }
                            continue;
                        } else {
                            return Some(next_pos);
                        }
                    } else {
                        self.current_step_cursor += 1;
                    }
                } else {
                    self.current_segment_cursor += 1;
                    continue;
                }
            } else {
                self.current_feature_cursor += 1;
                continue;
            }
        }
        return None;
    }

    fn get_current_segment(&self) -> Option<&NavigationFeatureSegment> {
        let route = self.navigation_route.as_ref().unwrap();

        if let Some(feature) = route.features.get(self.current_feature_cursor) {
            return feature.properties.segments.get(self.current_segment_cursor);
        }
        return None;
    }

    fn get_geometry_cordinates(&self, way_point: usize) -> Option<&Vec<f64>> {
        let route = self.navigation_route.as_ref().unwrap();

        if let Some(feature) = route.features.get(self.current_feature_cursor) {
            return feature.geometry.coordinates.get(way_point);
        }

        return None;
    }

    fn get_feature(&self) -> Option<&NavigationFeature> {
        let route = self.navigation_route.as_ref().unwrap();
        return route.features.get(self.current_feature_cursor);
    }

    fn get_step(&self) -> Option<&NavigationFeatureStep> {
        let route = self.navigation_route.as_ref().unwrap();

        if let Some(feature) = route.features.get(self.current_feature_cursor) {
            if let Some(segment) = feature.properties.segments.get(self.current_segment_cursor) {
                if let Some(step) = segment.steps.get(self.current_step_cursor) {
                    return Some(step);
                }
            }
        }
        return None;
    }

    pub fn get_bearing() {}

    pub fn get_current_position(&self) -> &GeoPosition {
        return &self.current_position;
    }

    pub fn get_current_lat() {}

    pub fn get_current_lon() {}

    pub async fn load_route(&mut self, destination: GeoPosition) {
        let client = reqwest::Client::new();
        let request = NavigationInfo::new(&self.current_position, &destination, "driving-car");
        let response = client
            .post(format!("{GEO_SERVICE_URL}/navigation"))
            .json(&request)
            .send()
            .await;

        match response {
            Ok(r) => {
                self.navigation_route = r.json::<NavigationCollection>().await.ok();
                println!("some{:?}", self.navigation_route.is_some());
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
