//! Server request for a ChargePoint to update it's configuration with a Key/Val pair.
//!
//! # Behaviour
//! This request contains a key-value pair, where "key" is the name of the configuration setting to change and "value" contains the new setting for the configuration setting.
//!
//! Examples of Change Configuration requests to which a Charge Point responds with a ChangeConfiguration.conf with a status of 'Rejected' are requests with out-of-range values and requests with values that do not conform to an expected format.
//!
//! If a key value is defined as a CSL, it MAY be accompanied with a \[KeyName\] MaxLength key, indicating the max length of the CSL in items. If this key is not set, a safe value of 1 (one) item SHOULD be assumed.
//! # Response
//! Upon receipt of a ChangeConfiguration.req Charge Point SHALL reply with a ChangeConfiguration.conf indicating whether it was able to apply the
//! change to its configuration. Content of "key" and "value" is not prescribed. The Charge Point SHALL set the status field in the ChangeConfiguration.conf according to the following rules:
//!
//! 1. If the change was applied successfully, and the change if effective immediately, the Charge Point SHALL respond with a status 'Accepted'.
//! 2. If the change was applied successfully, but a reboot is needed to make it effective, the Charge Point SHALL respond with status 'RebootRequired'.
//! 3. If "key" does not correspond to a configuration setting supported by Charge Point, it SHALL respond with a status 'NotSupported'.
//! 4. If the Charge Point did not set the configuration, and none of the previous statuses applies, the Charge Point SHALL respond with status 'Rejected'.

use crate::validation_macros::{self, json_validate};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/ChangeConfiguration.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the ChangeConfiguration.req PDU sent by Central System to Charge Point. It is RECOMMENDED
/// that the content and meaning of the 'key' and 'value' fields is agreed upon between Charge Point and Central System.
pub struct ChangeConfigurationRequest {
    /// Required. The name of the configuration setting to change.
    pub key: String,
    /// Required. The new value as string for the setting.
    pub value: String,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/ChangeConfigurationResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the ChangeConfiguration.conf PDU returned from Charge Point to Central System.
pub struct ChangeConfigurationResponse {
    /// Required. Returns whether configuration change has been accepted.
    pub status: ChangeConfigurationStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Status returned in response to ChangeAvailability.req.

pub enum ChangeConfigurationStatus {
    /// Configuration key is supported and setting has been changed.
    Accepted,
    /// Configuration key is supported, but setting could not be changed.
    Rejected,
    /// Configuration key is supported and setting has been changed, but change will be available after reboot (Charge Point will not reboot itself)
    RebootRequired,
    /// Configuration key is not supported.
    NotSupported,
}
