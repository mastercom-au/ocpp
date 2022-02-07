//! Update from charge point to inform the Central System about the status of a diagnostics upload.
//!
//! # Behaviour
//! The Charge Point SHALL send a DiagnosticsStatusNotification.req PDU to inform the Central System that the upload
//! of diagnostics is busy or has finished successfully or failed. The Charge Point SHALL only send the status Idle
//! after receipt of a TriggerMessage for a Diagnostics Status Notification, when it is not busy uploading diagnostics.
//!
//! # Response
//! Upon receipt of a DiagnosticsStatusNotification.req PDU, the Central System SHALL respond with a
//! DiagnosticsStatusNotification.conf.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/DiagnosticsStatusNotification.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the DiagnosticsStatusNotification.req PDU sent by the Charge Point to the Central System
pub struct DiagnosticStatusNotificationRequest {
    /// Required. This contains the status of the diagnostics upload.
    pub status: DiagnosticsStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Status in DataTransfer.conf.
pub enum DiagnosticsStatus {
    /// Charge Point is not performing diagnostics related tasks. Status Idle SHALL only be used as in a DiagnosticsStatusNotification.req that was triggered by a TriggerMessage.req
    Idle,
    /// Diagnostics information has been uploaded.
    Uploaded,
    /// Uploading of diagnostics failed.
    UploadFailed,
    /// File is being uploaded.
    Uploading,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/DiagnosticsStatusNotificationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the DataTransfer.conf PDU sent by the Charge Point to the Central System or vice versa in response to a DataTransfer.req PDU.
pub struct DiagnosticStatusNotificationResponse {}
