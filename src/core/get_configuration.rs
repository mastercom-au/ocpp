use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Requests/Core/GetConfiguration.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationRequest {
    pub key: Vec<String>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/Core/GetConfiguration.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse {
    pub configuration_key: Option<Vec<GetConfigConfigurationKey>>,
    pub unknown_key: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigConfigurationKey {
    pub key: String,
    pub readonly: bool,
    pub value: Option<String>,
}
