use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionRequest {
    pub transaction_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionResponse {
    pub status: RStopStatus,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum RStopStatus {
    Accepted,
    Rejected,
}
