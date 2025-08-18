fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["./proto/streamgate/v1/message_service.proto"], &["./proto"])
        .unwrap();
}
