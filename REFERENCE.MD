-- https://docs.confluent.io/confluent-cli/current/install.html  -- install confluent-cli for managing schemas in cloud

add new schema:
curl -X POST -H "Content-Type: application/vnd.schemaregistry.v1+json"   --data '{"schemaType":"PROTOBUF", "schema":'"$(cat update_driver_position.proto | jq -Rs .)"'}'   http://localhost:8081/subjects/gps_driver_position/versions


TODO:
write scripts to use
protoc --proto_path=./proto --csharp_out=./gps_service/Api/Messages/Protos/ ./proto/*.proto
