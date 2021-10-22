use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST --------------------------
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub status: BootNotificationStatus,
    pub current_time: DateTime<Utc>,
    pub interval: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BootNotificationStatus {
    Accepted,
    Pending,
    Rejected,
}
