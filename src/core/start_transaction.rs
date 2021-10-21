use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionRequest {
    pub connector_id: u32,
    pub id_tag: String,
    pub meter_start: i32,
    pub reservation_id: Option<i32>,
    pub timestamp: String,
}
