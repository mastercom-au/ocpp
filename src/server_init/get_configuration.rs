//! Server request for a ChargePoint to send it’s current configuration
//!
//! # Behaviour
//! The number of configuration keys requested in a single PDU MAY be limited by the Charge Point. This maximum can be retrieved by reading
//! the configuration key GetConfigurationMaxKeys.
//!
//! # Response
//! If the list of keys in the request PDU is empty or missing (it is optional), the Charge Point SHALL return a list of all configuration
//! settings in GetConfiguration.conf. Otherwise Charge Point SHALL return a list of recognized keys and their corresponding values and read-only
//! state. Unrecognized keys SHALL be placed in the response PDU as part of the optional unknown key list element of GetConfiguration.conf.
//!

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/GetConfiguration.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationRequest {
    pub key: Option<Vec<String>>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/GetConfigurationResponse.json")]
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
