//! Update from charge point to inform the Central System about the status of a firmware update.
//!
//! # Behaviour
//! The Charge Point SHALL send a FirmwareStatusNotification.req PDU for informing the Central System about the progress of
//! the downloading and installation of a firmware update. The Charge Point SHALL only send the status Idle after receipt of a
//! TriggerMessage for a Firmware Status Notification, when it is not busy downloading/installing firmware.
//!
//! # Response
//! Upon receipt of a FirmwareStatusNotification.req PDU, the Central System SHALL respond with a FirmwareStatusNotification.conf. The
//! FirmwareStatusNotification.req PDUs SHALL be sent to keep the Central System updated with the status of the update process,
//! started by the Central System with a FirmwareUpdate.req PDU.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/FirmwareStatusNotification.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the FirmwareStatusNotifitacion.req PDU sent by the Charge Point to the Central System
pub struct FirmwareStatusNotificationRequest {
    /// Required. This contains the progress status of the firmware installation.
    pub status: FirmwareNotificationStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
/// Status of a firmware download as reported in FirmwareStatusNotification.req.
pub enum FirmwareNotificationStatus {
    /// New firmware has been downloaded by Charge Point.
    Downloaded,
    /// Charge point failed to download firmware.
    DownloadFailed,
    /// Firmware is being downloaded.
    Downloading,
    /// Charge Point is not performing firmware update related tasks. Status Idle SHALL only be used as in a FirmwareStatusNotification.req that was triggered by a TriggerMessage.req
    Idle,
    /// Installation of new firmware has failed.
    InstallationFailed,
    /// Firmware is being installed.
    Installing,
    /// New firmware has successfully been installed in charge point.
    Installed,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/FirmwareStatusNotificationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the FirmwareStatusNotification.conf PDU sent by the Central System to the Charge Point in response to a FirmwareStatusNotification.req PDU.
pub struct FirmwareStatusNotificationResponse {}
