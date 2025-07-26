use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NavigationCollection {
    #[serde(rename = "type")]
    pub collection_type: String,
    pub bbox: Vec<f64>,
    pub features: Vec<NavigationFeature>,
    pub metadata: Metadata
}

#[derive(Deserialize, Debug)]
pub struct NavigationFeature {
    pub bbox: Vec<f64>,
    #[serde(rename = "type")]
    pub feature_type: String,
    pub properties: NavigationFeatureProperties,
    pub geometry: NavigationFeatureGeometry
}

#[derive(Deserialize, Debug)]
pub struct NavigationFeatureProperties {
    pub segments:  Vec<NavigationFeatureSegment>,
    pub way_points: Vec<usize>,
    pub summary: NavigationFeaturePropertySummary

}

#[derive(Deserialize, Debug)]
pub struct NavigationFeatureSegment {
    pub distance: f64,
    pub duration: f64,
    pub steps: Vec<NavigationFeatureStep>,
}

#[derive(Deserialize, Debug)]
pub struct NavigationFeatureStep {
    pub distance: f64,
    pub duration: f64,
    #[serde(rename = "type")]
    pub step_type: i32,
    pub instruction: String,
    pub name: String,
    pub way_points: Vec<usize> 
                            
}

#[derive(Deserialize, Debug)]
pub struct NavigationFeaturePropertySummary {
    pub distance: f64,
    pub duration: f64
}

#[derive(Deserialize, Debug)]
pub struct NavigationFeatureGeometry {
    pub coordinates: Vec<Vec<f64>>,
    #[serde(rename = "type")]
    pub geometry_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub attribution: String,
    pub service: String,
    pub timestamp: u64,
    pub query: Query,
    pub engine: Engine,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    pub coordinates: Vec<Vec<f64>>,
    pub profile: String,
    #[serde(rename = "profileName")]
    pub profile_name: String,
    pub format: String,
}

#[derive(Deserialize, Debug)]
pub struct Engine {
    pub version: String,
    pub build_date: String,
    pub graph_date: String,
}