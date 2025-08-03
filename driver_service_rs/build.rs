fn main() -> Result<(), Box<dyn std::error::Error>> {
   tonic_build::configure()
    .compile_protos(
        &["proto/trips/v1/trip.proto", "proto/trips/v1/trips_finder.proto"],
        &["proto"],
    )?;

    Ok(())
}
