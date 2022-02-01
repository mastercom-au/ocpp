use crate::common_types::{ChargingRateUnit, SimpleStatus};
use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/GetCompositeSchedule.json")]
#[skip_serializing_none]
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

#[skip_serializing_none]
#[json_validate("../json_schemas/GetCompositeScheduleResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse {
    pub status: SimpleStatus,
    pub connector_id: Option<u32>,
    pub charging_schedule: Option<GetCompChargingSchedule>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompChargingSchedule {
    pub duration: Option<u32>,
    pub start_schedule: Option<DateTime<Utc>>,
    pub charging_rate_unit: ChargingRateUnit,
    pub charging_schedule_period: Vec<GetCompChargingSchedulePeriod>,
    pub min_charging_rate: Option<f32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCompChargingSchedulePeriod {
    pub start_period: u32,
    pub limit: f32,
    pub number_phases: Option<u32>,
}
