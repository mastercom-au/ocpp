use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsRequest {
    pub location: String,
    pub retries: Option<u32>,
    pub retry_interval: Option<u32>,
    pub start_time: Option<DateTime<Utc>>,
    pub stop_time: Option<DateTime<Utc>>,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsResponse{
    pub file_name: String,
}