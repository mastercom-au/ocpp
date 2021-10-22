use serde::{Deserialize, Serialize};
use crate::common_types::{ChargingProfile};

/* Structure

ConnectorId u32
csChargingProfile struct
    chargingProfileId u32
    transactionId u32
    stackLevel u32
    chargingProfilePurpose String Enum
    chargingprofileKind String Enum
    recurrencyKind String Enum
    validFrom datetime<utc>
    validTo datetime<utc>
    chargingSchedule struct
        duration u32
        startSchedule date<utc>
        chargingRateUnit String enum
        minChargingRate f32 (increment 0.1)
        chargingSchedulePeriod Vec<obj>
            startPeriod u32
            limit f32 (increment 0.1)
            numberPhases u32
*/

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    pub connector_id: Option<u32>,
    pub cs_charging_profile: Option<ChargingProfile>,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse {
    pub status: SetChargingProfileStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SetChargingProfileStatus {
    Accepted,
    Rejected,
    NotSupported,
}
