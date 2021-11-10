use chrono::{DateTime, Utc};
use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};

use crate::error::ValidateError;
use crate::validate::JsonValidate;

const REQUEST_SCHEMA: &str = include_str!("../json_schemas/Requests/Core/BootNotification.json");
const RESPONSE_SCHEMA: &str = include_str!("../json_schemas/Responses/Core/BootNotification.json");

lazy_static! {
    static ref RESPONSE_JSON: serde_json::Value =
        serde_json::from_str(RESPONSE_SCHEMA).expect(&format!("Valid File: {}", RESPONSE_SCHEMA));
    static ref RESPONSE_VALIDATOR: jsonschema::JSONSchema =
        JSONSchema::compile(&RESPONSE_JSON).expect(&format!("Valid File: {}", RESPONSE_SCHEMA));
    static ref REQUEST_JSON: serde_json::Value =
        serde_json::from_str(REQUEST_SCHEMA).expect(&format!("Valid File: {}", REQUEST_SCHEMA));
    static ref REQUEST_VALIDATOR: jsonschema::JSONSchema =
        JSONSchema::compile(&REQUEST_JSON).expect(&format!("Valid File: {}", REQUEST_SCHEMA));
}

impl JsonValidate for BootNotificationRequest {
    fn validate(&self) -> Result<(), ValidateError> {
        self.generic_validate(&*REQUEST_VALIDATOR)?;
        Ok(())
    }
}

impl JsonValidate for BootNotificationResponse {
    fn validate(&self) -> Result<(), ValidateError> {
        self.generic_validate(&*RESPONSE_VALIDATOR)?;
        Ok(())
    }
}

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
