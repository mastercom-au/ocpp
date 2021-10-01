use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    pub id: u32,
    pub charging_profile_purpose: ChargingProfilePurpose,
    pub stackLevel: u32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}
