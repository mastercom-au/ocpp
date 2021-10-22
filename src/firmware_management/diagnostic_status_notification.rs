use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticStatusNotificationRequest {
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Status {
    Idle,
    Uploaded,
    UploadFailed,
    Uploading,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticStatusNotificationResponse{}