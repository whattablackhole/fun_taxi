fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all=\"camelCase\")]")
        .compile_protos(
            &[
                "proto/trips/v1/trip.proto",
                "proto/trips/v1/trips_finder.proto",
                "proto/trips/v1/trip_state_events/trip_assigned_to_driver.proto",
                "proto/trips/v1/trip_state_events/driver_trip_accepted.proto",
                "proto/streamgate/v1/message_service.proto",
            ],
            &["./proto"],
        )?;

    Ok(())
}
