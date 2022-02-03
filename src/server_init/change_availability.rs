//! Server request for a ChargePoint to change it's availability
//!
//! # Behaviour
//! A Charge Point is considered available (“operative”) when it is charging or ready for charging. A Charge Point is considered unavailable when it
//! does not allow any charging. The Central System SHALL send a ChangeAvailability.req PDU for requesting a Charge Point to change its availability.
//! The Central System can change the availability to available or unavailable.
//!
//! In the event that Central System requests Charge Point to change to a status it is already in, Charge Point SHALL respond with availability
//! status ‘Accepted’.
//!
//! When an availability change requested with a ChangeAvailability.req PDU has happened, the Charge Point SHALL inform Central System of its new
//! availability status with a StatusNotification.req as described there.
//!
//! In the case the ChangeAvailability.req contains ConnectorId = 0, the status change applies to the Charge Point and all Connectors.
//!
//! *States are persistent* i.e. Connector set to Unavailable shall persist a reboot.
//!
//! # Response
//! Upon receipt of a ChangeAvailability.req PDU, the Charge Point SHALL respond with a ChangeAvailability.conf PDU. The response PDU SHALL
//! indicate whether the Charge Point is able to change to the requested availability or not. When a transaction is in progress Charge Point
//! SHALL respond with availability status 'Scheduled' to indicate that it is scheduled to occur after the transaction has finished.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/ChangeAvailability.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    pub connector_id: u32,
    pub r#type: ChangeAvailabilityType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum ChangeAvailabilityType {
    Inoperative,
    Operative,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/ChangeAvailabilityResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    pub status: ChangeAvailabilityStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum ChangeAvailabilityStatus {
    Accepted,
    Rejected,
    Scheduled,
}