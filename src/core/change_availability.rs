use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Accepted,
    Rejected,
    Scheduled,
}
