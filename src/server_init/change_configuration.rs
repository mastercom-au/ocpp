use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/ChangeConfiguration.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfigurationRequest {
    pub key: String,
    pub value: String,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/ChangeConfigurationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfigurationResponse {
    pub status: ChangeConfigurationStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum ChangeConfigurationStatus {
    Accepted,
    Rejected,
    RebootRequired,
    NotSupported,
}
