//! Server request for a ChargePoint to clear it’s Charging Profile
//!
//! The Charge Point SHALL respond with a ClearChargingProfile.conf PDU specifying whether it was able to process the request.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/ClearChargingProfile.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    pub id: Option<u32>,
    pub charging_profile_purpose: Option<ChargingProfilePurpose>,
    pub stack_level: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/ClearChargingProfileResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    pub status: ClearChargeProfileStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum ClearChargeProfileStatus {
    Accepted,
    Unknown,
}