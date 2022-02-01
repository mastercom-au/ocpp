use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Core/GetConfiguration.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationRequest {
    pub key: Option<Vec<String>>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Core/GetConfiguration.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse {
    pub configuration_key: Option<Vec<GetConfigConfigurationKey>>,
    pub unknown_key: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigConfigurationKey {
    pub configuration_key: Option<ConfigurationKey>,
    pub unknown_key: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationKey {
    pub key: String,
    pub readonly: bool,
    pub value: Option<String>,
}
