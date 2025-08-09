use redis::{
    AsyncCommands,
    aio::MultiplexedConnection,
    geo::{self, Coord, RadiusSearchResult},
};
use tonic::{Request, Response, Status};

use crate::protos::gps::{
    DriverAndPosition, SearchDriversReply, SearchDriversRequest,
    drivers_finder_service_server::DriversFinderService,
};

pub struct DriversFinderServiceImpl {
    db: MultiplexedConnection,
}

impl DriversFinderServiceImpl {
    pub fn new(db: MultiplexedConnection) -> Self {
        Self { db: db }
    }
}

#[tonic::async_trait]
impl DriversFinderService for DriversFinderServiceImpl {
    async fn search_drivers_by_position(
        &self,
        request: Request<SearchDriversRequest>,
    ) -> Result<Response<SearchDriversReply>, Status> {
        let mut db = self.db.clone();

        let req: SearchDriversRequest = request.into_inner();
        let search_results = {
            let result: Result<Vec<RadiusSearchResult>, redis::RedisError> = db
                .geo_radius(
                    "drivers_position",
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
                    "No drivers found within provided coordinates",
                ));
            }
        };
        let mut reply = SearchDriversReply::default();

        search_results.iter().for_each(|r| {
            reply.drivers_and_positions.push(DriverAndPosition {
                driver_id: r.name.clone(),
                lat: r.coord.as_ref().unwrap().latitude,
                lon: r.coord.as_ref().unwrap().longitude,
            });
        });

        return Ok(Response::new(reply));
    }
}
