use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Requests/Core/DataTransfer.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    pub vendor_id: String,
    pub message_id: Option<String>,
    pub data: Option<String>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/Core/DataTransfer.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    pub status: DataTransferStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DataTransferStatus {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}
