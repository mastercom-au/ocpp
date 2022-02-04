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
/// Field definition of the GetConfiguration.req PDU sent by the Central System to the Charge Point.
pub struct GetConfigurationRequest {
    /// Optional. List of keys for which the configuration value is requested. Keys are separated by ','
    pub key: Option<Vec<String>>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/GetConfigurationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the GetConfiguration.conf PDU sent by Charge Point the to the Central System in response to a GetConfiguration.req.
pub struct GetConfigurationResponse {
    /// Optional. List of requested or known keys
    pub configuration_key: Option<Vec<KeyValue>>,
    /// Optional. Requested keys that are unknown
    pub unknown_key: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Contains information about a specific configuration key. It is returned in GetConfiguration.conf.
pub struct KeyValue {
    /// Key
    pub key: String,
    /// Required. False if the value can be set with the ChangeConfiguration message.
    pub readonly: bool,
    /// Optional. If key is known but not set, this field may be absent.
    pub value: Option<String>,
}
