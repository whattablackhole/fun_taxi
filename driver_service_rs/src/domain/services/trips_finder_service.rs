use std::time::Duration;

use tokio::time::sleep;
use tonic::transport::{Channel, Endpoint, Error};
use tonic::Status;

use super::fun_taxi_messages_proto_trips::Trip;
use super::fun_taxi_messages_proto_trips::trips_finder_service_client::TripsFinderServiceClient;
use super::fun_taxi_messages_proto_trips::AvailableTripsRequest;

pub struct TripsFinderService {
    client: TripsFinderServiceClient<tonic::transport::Channel>,
}

impl TripsFinderService {
    pub async fn new(dst: String) -> Result<Self, Error> {
      
        let client = Self::connect_with_infinite_retry(&dst).await;
        println!("--- Inited trips finder service successfully ---");
        Ok(Self { client })
    }


    pub async fn get_available_trips(&mut self, lat: f64, lon: f64, radius: i32) -> Result<Vec<Trip>, Status> {
        let request = tonic::Request::new(AvailableTripsRequest {
            lat,
            lon,
            radius
        });

        let mut response = self.client.get_available_trips(request).await?;

        Ok(response.get_mut().to_owned().trips)
    }

   pub async fn connect_with_infinite_retry(dst: &str) -> TripsFinderServiceClient<Channel> {
    let mut retry_delay = Duration::from_secs(1);

    loop {
        let endpoint = Endpoint::from_shared(dst.to_string())
            .expect("Invalid URI")
            .timeout(Duration::from_secs(5));

        match endpoint.connect().await {
            Ok(channel) => {
                println!("Connected to trip coordinator service!");
                return TripsFinderServiceClient::new(channel);
            }
            Err(e) => {
                eprintln!(
                    "Failed to connect to trip coordinator: {}. Retrying in {:?}...",
                    e, retry_delay
                );
                sleep(retry_delay).await;
                retry_delay = retry_delay.min(Duration::from_secs(30)) * 2;
            }
        }
    }
}
}
