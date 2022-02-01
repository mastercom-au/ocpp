use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/DiagnosticsStatusNotification.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticStatusNotificationRequest {
    pub status: DiagnosticNotificationStatus,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum DiagnosticNotificationStatus {
    Idle,
    Uploaded,
    UploadFailed,
    Uploading,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/DiagnosticsStatusNotificationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticStatusNotificationResponse {}
