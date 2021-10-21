use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/* Structure

ConnectorId u32
idTag String
chargingProfile struct
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransactionRequest {
    pub connector_id: Option<u32>,
    pub id_tag: String,
    pub charging_profile: Option<StartChargingProfile>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartChargingProfile {
    charging_profile_id: u32,
    transaction_id: Option<u32>,
    stack_level: u32,
    charging_profile_purpose: StartChargingProfilePurpose,
    charging_profile_kind: StartChargingProfileKind,
    recurrency_kind: Option<StartRecurrencyKind>,
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
pub enum StartChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StartChargingProfileKind {
    Absolute,
    Recurring,
    Relative,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StartRecurrencyKind {
    Daily,
    Weekly,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChargingRateUnit {
    A,
    W,
}
