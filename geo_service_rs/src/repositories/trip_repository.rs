use redis::aio::MultiplexedConnection;

use crate::protos::trips::{TripGeoPositionAddCommand, TripGeoPositionRemoveCommand};

// TODO: decouple cmd from repo...
pub struct TripRepository;

impl TripRepository {
    pub async fn add_new_trip(
        cmd: &TripGeoPositionAddCommand,
        connection: &mut MultiplexedConnection,
    ) -> redis::RedisResult<()> {
        let mut pipe = redis::pipe();
        pipe.atomic()
            .cmd("GEOADD")
            .arg("available_trips:start")
            .arg(&cmd.start_lat)
            .arg(&cmd.start_lon)
            .arg(&cmd.id)
            .cmd("GEOADD")
            .arg("available_trips:end")
            .arg(&cmd.end_lat)
            .arg(&cmd.end_lon)
            .arg(&cmd.id);

        pipe.query_async::<()>(connection).await?;
        Ok(())
    }

    pub async fn remove_trip(
        cmd: &TripGeoPositionRemoveCommand,
        connection: &mut MultiplexedConnection,
    ) -> redis::RedisResult<()> {
        let mut pipe = redis::pipe();
        pipe.atomic()
            .cmd("ZREM")
            .arg("available_trips:start")
            .arg(&cmd.id)
            .cmd("ZREM")
            .arg("available_trips:end")
            .arg(&cmd.id);

        pipe.query_async::<()>(connection).await?;
        Ok(())
    }
}