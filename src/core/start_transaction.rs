use chrono::{DateTime, Utc};
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionResponse {
    pub id_tag_info: IdTagInfo,
    pub transaction_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    pub expiry_date: Option<DateTime<Utc>>,
    pub parent_id_tag: Option<String>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Accepted,
    Rejected,
    Expired,
    Invalid,
    ConcurrentTx,
}
