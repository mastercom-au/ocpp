use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    connector_id: u32,
    duration: u32,
    charging_rate_unit: Option<ChargingRateUnit>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ChargingRateUnit {
    A,
    W,
}

// -------------------------- RESPONSE --------------------------
/*Structure
status String enum
connectorId u32
chargingSchedule struct
    duration u32
    startSchedule datetime utc
    chargingRateUnit string enum
    chargingSchedulePeriod array structs
        startPeriod u32
        limit f32
        numberPhases u32
    minChargingRate float
*/



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse{
    pub status: Status,
    pub connector_id: u32,
    pub charging_schedule: ChargingSchedule,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status{
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedule{
    pub duration: u32,
    pub start_schedule: DateTime<Utc>,
    pub charging_rate_unit: ChargingRateUnit,
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    pub min_charging_rate: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriod{
    pub start_period: u32,
    pub limit: f32,
    pub number_phases: u32,
}