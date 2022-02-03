//! Server request for a ChargePoint to send it's local auth list version.
//!
//! # Behaviour
//! In order to support synchronisation of the Local Authorization List, Central System can request a Charge Point for the
//! version number of the Local Authorization List. The Central System SHALL send a GetLocalListVersion.req PDU to request this value.
//!
//! # Response
//! Upon receipt of a GetLocalListVersion.req PDU Charge Point SHALL respond with a GetLocalListVersion.conf PDU containing
//! the version number of its Local Authorization List. A version number of 0 (zero) SHALL be used to indicate that the local
//! authorization list is empty, and a version number of -1 SHALL be used to indicate that the Charge Point does not support Local
//! Authorization Lists.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST --------------------------
#[json_validate("../json_schemas/GetLocalListVersion.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/GetLocalListVersionResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    /// Required. This contains the current version number of the local authorization list in the Charge Point.
    pub list_version: u32,
}
