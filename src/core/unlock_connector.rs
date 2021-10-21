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
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Unlocked,
    UnlockFailed,
    NotSupported,
}
