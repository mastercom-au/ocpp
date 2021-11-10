use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Requests/Core/ChangeConfiguration.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfigurationRequest {
    pub key: String,
    pub value: String,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/Core/ChangeConfiguration.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfigurationResponse {
    pub status: ChangeConfigurationStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChangeConfigurationStatus {
    Accepted,
    Rejected,
    RebootRequired,
    NotSupported,
}
