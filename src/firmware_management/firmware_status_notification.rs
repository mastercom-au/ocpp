use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/FirmwareManagement/FirmwareStatusNotification.json")]
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
#[json_validate("../json_schemas/FirmwareManagement/FirmwareStatusNotification.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {}
