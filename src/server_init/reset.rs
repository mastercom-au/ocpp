//! Server request to reboot/reset ChargePoint
//!
//! # Behaviour
//! The Central System SHALL send a Reset.req PDU for requesting a Charge Point to reset itself. The Central System can request a hard or
//! a soft reset.
//!
//! After receipt of a Reset.req, The Charge Point SHALL send a StopTransaction.req for any ongoing transaction before performing the reset.
//! If the Charge Point fails to receive a StopTransaction.conf form the Central System, it shall queue the StopTransaction.req.
//!
//! At receipt of a soft reset, the Charge Point SHALL stop ongoing transactions gracefully and send StopTransaction.req for every ongoing
//! transaction. It should then restart the application software (if possible, otherwise restart the processor/controller).
//!
//! At receipt of a hard reset the Charge Point SHALL restart (all) the hardware, it is not required to gracefully stop ongoing transaction.
//! If possible the Charge Point sends a StopTransaction.req for previously ongoing transactions after having restarted and having been accepted
//! by the Central System via a BootNotification.conf. This is a last resort solution for a not correctly functioning Charge Points, by sending
//! a "hard" reset, (queued) information might get lost.
//! *States are persistent* i.e. Connector set to Unavailable shall persist a reboot.
//!
//! # Response
//! Upon receipt of a Reset.req PDU, the Charge Point SHALL respond with a Reset.conf PDU. The response PDU SHALL include
//! whether the Charge Point will attempt to reset itself.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

pub use crate::common_types::SimpleStatus;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Reset.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the Reset.req PDU sent by the Central System to the Charge Point.
pub struct ResetRequest {
    /// Required. This contains the type of reset that the Charge Point should perform.
    pub r#type: ResetType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Type of reset requested by Reset.req.
pub enum ResetType {
    /// Restart (all) the hardware, the Charge Point is not required to gracefully stop ongoing transaction. If possible the Charge Point sends a StopTransaction.req
    /// for previously ongoing transactions after having restarted and having been accepted by the Central System via a BootNotification.conf. This is a last resort
    /// solution for a not correctly functioning Charge Point, by sending a "hard" reset, (queued) information might get lost.
    Hard,
    /// Stop ongoing transactions gracefully and sending StopTransaction.req for every ongoing transaction. It should then restart the application software (if
    /// possible, otherwise restart the processor/controller).
    Soft,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/ResetResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the Reset.conf PDU sent by the Charge Point to the Central System in response to a Reset.req PDU.
pub struct ResetResponse {
    /// Required. This indicates whether the Charge Point is able to perform the reset.
    pub status: SimpleStatus,
}
