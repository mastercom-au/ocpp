use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Requests/Core/StartTransaction.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionRequest {
    pub connector_id: u32,
    pub id_tag: String,
    pub meter_start: i32,
    pub reservation_id: Option<i32>,
    pub timestamp: DateTime<Utc>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/Core/StartTransaction.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionResponse {
    pub id_tag_info: StartTransactionIdTagInfo,
    pub transaction_id: u32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionIdTagInfo {
    pub expiry_date: Option<DateTime<Utc>>,
    pub parent_id_tag: Option<String>,
    pub status: StartTransactionStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StartTransactionStatus {
    Accepted,
    Rejected,
    Expired,
    Invalid,
    ConcurrentTx,
}
