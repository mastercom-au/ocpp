//! Server request for a ChargePoint to clear it’s Charging Profile
//!
//! The Charge Point SHALL respond with a ClearChargingProfile.conf PDU specifying whether it was able to process the request.

use crate::macros::{self, json_validate};
use crate::ChargingProfilePurpose;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/ClearChargingProfile.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the ClearChargingProfile.req PDU sent by the Central System to the Charge Point.
///
/// The Central System can use this message to clear (remove) either a specific charging profile (denoted by id) or a selection
/// of charging profiles that match with the values of the optional connectorId, stackLevel and chargingProfilePurpose fields.
pub struct ClearChargingProfileRequest {
    /// Optional. The ID of the charging profile to clear.
    pub id: Option<u32>,
    /// Optional. Specifies the ID of the connector for which to clear charging profiles. A connectorId of zero (0) specifies the charging profile for the overall Charge Point.
    /// Absence of this parameter means the clearing applies to all charging profiles that match the other criteria in the request.
    pub connector_id: Option<u32>,
    /// Optional. Specifies to purpose of the charging profiles that will be cleared, if they meet the other criteria in the request.
    pub charging_profile_purpose: Option<ChargingProfilePurpose>,
    /// Optional. specifies the stackLevel for which charging profiles will be cleared, if they meet the other criteria in the request
    pub stack_level: Option<u32>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/ClearChargingProfileResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the ClearChargingProfile.conf PDU sent by the Charge Point to the Central System in response to a ClearChargingProfile.req PDU.
pub struct ClearChargingProfileResponse {
    /// Required. Indicates if the Charge Point was able to execute the request.
    pub status: ClearChargeProfileStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Status returned in response to ClearChargingProfile.req.
pub enum ClearChargeProfileStatus {
    /// Request has been accepted and will be executed.
    Accepted,
    /// No Charging Profile(s) were found matching the request.
    Unknown,
}
