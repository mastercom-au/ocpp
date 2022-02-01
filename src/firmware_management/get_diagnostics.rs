use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/FirmwareManagement/GetDiagnostics.json")]
#[skip_serializing_none]
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
#[json_validate("../json_schemas/FirmwareManagement/GetDiagnosticsResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsResponse {
    pub file_name: String,
}
