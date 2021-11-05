use std::fs::read_to_string;

use chrono::{DateTime, Utc};
use jsonschema::{error::ValidationErrorKind, JSONSchema, ValidationError};

use crate::core::{JsonValidate, ValidateError};
use serde::{Deserialize, Serialize};
use serde_json::json;

const REQUEST_SCHEMA: &str = include_str!("../json_schemas/Requests/Core/BootNotification.json");
const RESPONSE_SCHEMA: &str = include_str!("../json_schemas/Responses/Core/BootNotification.json");

lazy_static! {
    static ref RESPONSE_JSON: serde_json::Value =
        serde_json::from_str(RESPONSE_SCHEMA).expect("Valid File");
    static ref RESPONSE_VALIDATOR: jsonschema::JSONSchema =
        JSONSchema::compile(&RESPONSE_JSON).expect("Valid Schema");
    static ref REQUEST_JSON: serde_json::Value =
        serde_json::from_str(REQUEST_SCHEMA).expect("Valid File");
    static ref REQUEST_VALIDATOR: jsonschema::JSONSchema =
        JSONSchema::compile(&REQUEST_JSON).expect("Valid Schema");
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

impl JsonValidate for BootNotificationRequest {
    fn validate(&self) -> Result<(), Vec<String>> {
        self.generic_validate(&*REQUEST_VALIDATOR)?;
        println!("Request Validated Succesfully");
        Ok(())
    }
}

impl JsonValidate for BootNotificationResponse {
    fn validate(&self) -> Result<(), Vec<String>> {
        self.generic_validate(&*RESPONSE_VALIDATOR)?;
        println!("Response Validated Succesfully");
        Ok(())
    }
}
