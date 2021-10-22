use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    pub connector_id: u32,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorResponse {
    pub status: UCStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum UCStatus {
    Unlocked,
    UnlockFailed,
    NotSupported,
}
