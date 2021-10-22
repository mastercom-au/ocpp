use crate::common_types::SimpleStatus;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionRequest {
    pub transaction_id: u32,
}


// -------------------------- RESPONSE --------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionResponse {
    pub status: SimpleStatus,
}

