use actix_web::web::Bytes;
use reqwest::Error;

pub struct OpenRouteApiService {
    base: String,
    api_key: String,
    client: reqwest::Client,
}

impl OpenRouteApiService {
    pub fn new(base: String, api_key: String) -> Self {
        return Self {
            base: base,
            api_key: api_key,
            client: reqwest::Client::new(),
        };
    }

    pub async fn get_navigation(&self, profile: &str, start: &str, end: &str) -> Result<Bytes, Error> {
        let key = &self.api_key;
        let base = &self.base;

        let url = format!("{base}/directions/{profile}?api_key={key}&start={start}&end={end}");
        let res = self.client.get(url).send().await?;

        return res.bytes().await;
    }
}
