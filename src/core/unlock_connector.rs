use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Core/UnlockConnector.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    pub connector_id: u32,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Core/UnlockConnector.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorResponse {
    pub status: UnlockConnectorStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum UnlockConnectorStatus {
    Unlocked,
    UnlockFailed,
    NotSupported,
}
