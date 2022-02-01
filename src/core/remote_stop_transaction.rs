use crate::common_types::SimpleStatus;
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Core/RemoteStopTransaction.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionRequest {
    pub transaction_id: u32,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Core/RemoteStopTransactionResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionResponse {
    pub status: SimpleStatus,
}
