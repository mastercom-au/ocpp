use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareNotificationStatus,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum FirmwareNotificationStatus {
    Downloaded,
    DownloadFailed,
    Downloading,
    Idle,
    InstallationFailed,
    Installing,
    Installed,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {}
