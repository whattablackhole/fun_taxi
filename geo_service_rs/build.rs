use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_prost_build::configure()
        .build_server(true)

        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all=\"camelCase\")]")

        .compile_protos(
            &[
                "proto/gps/v1/update_driver_position.proto",
                "proto/trips/v1/trip_state_commands/trip_geoposition_add.proto",
                "proto/trips/v1/trip.proto",
                "proto/trips/v1/trips_finder.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}