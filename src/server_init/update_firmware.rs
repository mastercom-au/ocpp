use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/UpdateFirmware.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    pub location: String,
    pub retries: Option<u32>,
    pub retrieve_date: DateTime<Utc>,
    pub retry_interval: Option<u32>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/UpdateFirmwareResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {}
