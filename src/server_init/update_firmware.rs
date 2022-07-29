//! Server request for a chargepoint to update it's firmware
//!
//! # Behaviour
//! Central System can notify a Charge Point that it needs to update its firmware. The Central System SHALL send an UpdateFirmware.req
//! PDU to instruct the Charge Point to install new firmware. The PDU SHALL contain a date and time after which the Charge Point is
//! allowed to retrieve the new firmware and the location from which the firmware can be downloaded.
//!
//! The Charge Point SHALL, if the new firmware image is "valid", install the new firmware as soon as it is able to.
//!
//! If it is not possible to continue charging during installation of firmware, it is RECOMMENDED to wait until Charging Session has
//! ended (Charge Point idle) before commencing installation. It is RECOMMENDED to set connectors that are not in use to UNAVAILABLE
//! while the Charge Point waits for the Session to end.
//!
//! # Response
//! Upon receipt of an UpdateFirmware.req PDU, the Charge Point SHALL respond with a UpdateFirmware.conf PDU. The Charge Point SHOULD
//! start retrieving the firmware as soon as possible after retrieve-date. During downloading and installation of the firmware, the
//! Charge Point MUST send FirmwareStatusNotification.req PDUs to keep the Central System updated with the status of the update process.

use crate::macros::{self, json_validate};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/UpdateFirmware.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the UpdateFirmware.req PDU sent by the Central System to the Charge Point.
pub struct UpdateFirmwareRequest {
    /// Required. This contains a string containing a URI pointing to a location from which to retrieve the firmware.
    pub location: String,
    /// Optional. This specifies how many times Charge Point must try to download the firmware before giving up. If this field is not
    /// present, it is left to Charge Point to decide how many times it wants to retry.
    pub retries: Option<u32>,
    /// Required. This contains the date and time after which the Charge Point is allowed to retrieve the (new) firmware.
    pub retrieve_date: DateTime<Utc>,
    /// Optional. The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charge Point
    /// to decide how long to wait between attempts.
    pub retry_interval: Option<u32>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/UpdateFirmwareResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the UpdateFirmware.conf PDU sent by the Charge Point to the Central System in response to a UpdateFirmware.req PDU.
pub struct UpdateFirmwareResponse {}
