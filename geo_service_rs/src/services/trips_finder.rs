use redis::{
    AsyncCommands,
    aio::MultiplexedConnection,
    geo::{self, Coord, RadiusSearchResult},
};
use tonic::{Request, Response, Status};

use crate::protos::trips::{
    AvailableTripsReply, AvailableTripsRequest, Trip,
    trips_finder_service_server::TripsFinderService,
};

pub struct TripsFinderServiceImpl {
    db: MultiplexedConnection,
}

impl TripsFinderServiceImpl {
    pub fn new(db: MultiplexedConnection) -> Self {
        Self { db: db }
    }
}

#[tonic::async_trait]
impl TripsFinderService for TripsFinderServiceImpl {
    async fn get_available_trips(
        &self,
        request: Request<AvailableTripsRequest>,
    ) -> Result<Response<AvailableTripsReply>, Status> {
        let mut db = self.db.clone();

        let req: AvailableTripsRequest = request.into_inner();
        let start_search_result = {
            let result: Result<Vec<RadiusSearchResult>, redis::RedisError> = db
                .geo_radius(
                    "available_trips:start",
                    req.lon,
                    req.lat,
                    req.radius.into(),
                    geo::Unit::Meters,
                    geo::RadiusOptions::default().with_coord(),
                )
                .await;

            if let Ok(search_result) = result {
                search_result
            } else {
                return Result::Err(Status::not_found(
                    "No trips found within provided coordinates",
                ));
            }
        };

        let end_positions = {
            let trip_ids: Vec<String> =
                start_search_result.iter().map(|r| r.name.clone()).collect();
            let end_positions: Result<Vec<Coord<f64>>, redis::RedisError> =
                db.geo_pos("available_trips:end", &trip_ids).await;

            if let Ok(search_result) = end_positions {
                search_result
            } else {
                return Result::Err(Status::not_found(
                    "No trips found within provided coordinates",
                ));
            }
        };

        if end_positions.len() != start_search_result.len() {
            return Result::Err(Status::data_loss(
                "Corrupted State: start and end positions are not matched",
            ));
        }

        let trips: Vec<Trip> = start_search_result
            .iter()
            .enumerate()
            .map(|(i, r)| Trip {
                end_lat: end_positions.get(i).unwrap().latitude,
                end_lon: end_positions.get(i).unwrap().longitude,
                start_lat: r.coord.as_ref().unwrap().latitude,
                start_lon: r.coord.as_ref().unwrap().longitude,
                id: r.name.clone(),
            })
            .collect();

        let reply = AvailableTripsReply { trips: trips };

        Ok(Response::new(reply))
    }
}
