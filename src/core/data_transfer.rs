use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    pub vendor_id: String,
    pub message_id: Option<String>,
    pub data: Option<String>,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    pub status: DTStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DTStatus {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}
