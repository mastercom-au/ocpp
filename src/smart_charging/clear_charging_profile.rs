use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    pub id: u32,
    pub charging_profile_purpose: ChargingProfilePurpose,
    pub stack_level: u32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    pub status: ClearChargeProfileStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ClearChargeProfileStatus {
    Accepted,
    Unknown,
}
