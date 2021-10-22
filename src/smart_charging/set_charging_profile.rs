use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub cs_charging_profile: Option<CsChargingProfile>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CsChargingProfile {
    charging_profile_id: u32,
    transaction_id: Option<u32>,
    stack_level: u32,
    charging_profile_purpose: ChargingProfilePurpose,
    charging_profile_kind: ChargingProfileKind,
    recurrency_kind: Option<RecurrencyKind>,
    valid_from: Option<DateTime<Utc>>,
    valid_to: Option<DateTime<Utc>>,
    charging_schedule: ProfileSchedule,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSchedule {
    pub duration: Option<u32>,
    pub start_schedule: Option<DateTime<Utc>>,
    pub charging_rate_unit: ChargingRateUnit,
    pub charging_schedule_period: Vec<ProfileSchedulePeriod>,
    pub min_charging_rate: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSchedulePeriod {
    pub start_period: u32,
    pub limit: f32, //increment 0.1
    pub number_phases: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChargingProfileKind {
    Absolute,
    Recurring,
    Relative,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum RecurrencyKind {
    Daily,
    Weekly,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChargingRateUnit {
    A,
    W,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse {
    pub status: SCPStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SCPStatus {
    Accepted,
    Rejected,
    NotSupported,
}
