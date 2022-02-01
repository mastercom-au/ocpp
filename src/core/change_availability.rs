use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Core/ChangeAvailability.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    pub connector_id: u32,
    pub r#type: ChangeAvailabilityType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChangeAvailabilityType {
    Inoperative,
    Operative,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Core/ChangeAvailability.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    pub status: ChangeAvailabilityStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChangeAvailabilityStatus {
    Accepted,
    Rejected,
    Scheduled,
}
