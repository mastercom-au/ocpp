use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Requests/FirmwareManagement/UpdateFirmware.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    pub location: String,
    pub retries: Option<u32>,
    pub retrieve_date: DateTime<Utc>,
    pub retry_interval: Option<u32>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/FirmwareManagement/UpdateFirmware.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {}
