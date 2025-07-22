use serde::Deserialize;
use uuid::Uuid;



#[derive(Deserialize)]
pub struct StartDriverDto {
    pub driver_id: Uuid,
}