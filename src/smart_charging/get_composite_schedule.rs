use crate::common_types::{ChargingRateUnit, SimpleStatus};
use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Requests/SmartCharging/GetCompositeSchedule.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    connector_id: u32,
    duration: u32,
    charging_rate_unit: Option<ChargingRateUnit>,
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

#[json_validate("../json_schemas/Responses/SmartCharging/GetCompositeSchedule.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse {
    pub status: SimpleStatus,
    pub connector_id: u32,
    pub charging_schedule: GetCompChargingSchedule,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompChargingSchedule {
    pub duration: u32,
    pub start_schedule: DateTime<Utc>,
    pub charging_rate_unit: ChargingRateUnit,
    pub charging_schedule_period: Vec<GetCompChargingSchedulePeriod>,
    pub min_charging_rate: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompChargingSchedulePeriod {
    pub start_period: u32,
    pub limit: f32,
    pub number_phases: u32,
}
