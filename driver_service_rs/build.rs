fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/trips/get_available_trips.proto")?;
    Ok(())
}