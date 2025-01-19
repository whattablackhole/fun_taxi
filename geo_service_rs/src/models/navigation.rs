use serde::Deserialize;

#[derive(Deserialize)]
pub struct NavigationInfo {
    pub profile: String,
    pub start: String,
    pub end: String,
}