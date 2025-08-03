
use tonic::transport::{Channel};
use tonic::Status;

use super::fun_taxi_messages_proto_trips::trips_finder_service_client::TripsFinderServiceClient;
use super::fun_taxi_messages_proto_trips::AvailableTripsRequest;
use super::fun_taxi_messages_proto_trips::Trip;

pub struct TripsFinderService {
    client: TripsFinderServiceClient<tonic::transport::Channel>,
}

impl TripsFinderService {
    pub fn new(channel: Channel) -> Self {
        let client = TripsFinderServiceClient::new(channel);
        println!("--- Inited trips finder service successfully ---");
        Self { client }
    }

    pub async fn get_available_trips(
        &mut self,
        lat: f64,
        lon: f64,
        radius: i32,
    ) -> Result<Vec<Trip>, Status> {
        let request = tonic::Request::new(AvailableTripsRequest { lat, lon, radius });

        let mut response = self.client.get_available_trips(request).await?;

        Ok(response.get_mut().to_owned().trips)
    }
}
