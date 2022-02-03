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

use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/UpdateFirmware.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    pub location: String,
    pub retries: Option<u32>,
    pub retrieve_date: DateTime<Utc>,
    pub retry_interval: Option<u32>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/UpdateFirmwareResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {}
