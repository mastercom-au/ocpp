use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/ChangeAvailability.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    pub connector_id: u32,
    pub r#type: ChangeAvailabilityType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum ChangeAvailabilityType {
    Inoperative,
    Operative,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/ChangeAvailabilityResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    pub status: ChangeAvailabilityStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum ChangeAvailabilityStatus {
    Accepted,
    Rejected,
    Scheduled,
}
