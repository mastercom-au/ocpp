use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    pub location: String,
    pub retries: Option<u32>,
    pub retrieve_date: DateTime<Utc>,
    pub retry_interval: Option<u32>,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {}
