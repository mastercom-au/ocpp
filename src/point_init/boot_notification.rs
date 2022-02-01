use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

// -------------------------- REQUEST --------------------------
#[json_validate("../json_schemas/BootNotification.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub charge_point_vendor: String,
    pub charge_point_model: String,
    pub charge_point_serial_number: Option<String>,
    pub charge_box_serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub meter_type: Option<String>,
    pub meter_serial_number: Option<String>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/BootNotificationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub status: BootNotificationStatus,
    pub current_time: DateTime<Utc>,
    pub interval: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum BootNotificationStatus {
    Accepted,
    Pending,
    Rejected,
}
