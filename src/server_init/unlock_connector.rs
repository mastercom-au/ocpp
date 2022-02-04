//! Server request for a ChargePoint to allow a connector to unlock from a vehicle
//!
//! # Behaviour
//! The purpose of this message: Help EV drivers that have problems unplugging their cable from the Charge Point in case of malfunction of the
//! Connector cable retention. When a EV driver calls the CPO help-desk, an operator could manually trigger the sending of an UnlockConnector.req
//! to the Charge Point, forcing a new attempt to unlock the connector. Hopefully this time the connector unlocks and the EV driver can unplug the cable and drive away.
//!
//! The UnlockConnector.req SHOULD NOT be used to remotely stop a running transaction, use the Remote Stop Transaction instead.
//!
//! # Response
//! Upon receipt of an UnlockConnector.req PDU, the Charge Point SHALL respond with a UnlockConnector.conf PDU. The response PDU SHALL
//! indicate whether the Charge Point was able to unlock its connector. If there was a transaction in progress on the specific connector,
//! then Charge Point SHALL finish the transaction first as described in Stop Transaction.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/UnlockConnector.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// This contains the field definition of the UnlockConnector.req PDU sent by the Central System to the Charge Point.
pub struct UnlockConnectorRequest {
    /// Required. This contains the identifier of the connector to be unlocked.
    pub connector_id: u32,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/UnlockConnectorResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// This contains the field definition of the UnlockConnector.conf PDU sent by the Charge Point to the Central System in response to an UnlockConnector.req PDU.
pub struct UnlockConnectorResponse {
    /// Required. This indicates whether the Charge Point has unlocked the connector.
    pub status: UnlockConnectorStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
/// Status in response to UnlockConnector.req.
pub enum UnlockConnectorStatus {
    /// Connector has successfully been unlocked.
    Unlocked,
    /// Failed to unlock the connector: The Charge Point has tried to unlock the connector and has detected that the connector is still locked or the unlock mechanism failed
    UnlockFailed,
    /// Charge Point has no connector lock, or ConnectorId is unknown.
    NotSupported,
}
