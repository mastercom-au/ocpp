use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationRequest {
    pub key: Vec<String>,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse {
    pub configuration_key: Option<Vec<ConfigurationKey>>,
    pub unknown_key: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationKey {
    pub key: String,
    pub readonly: bool,
    pub value: Option<String>,
}
