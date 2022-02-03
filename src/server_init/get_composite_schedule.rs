//! Server request for a ChargePoint to send it's Composite Schedule
//!
//! # Behaviour
//! The reported schedule, in the GetCompositeSchedule.conf PDU, is the result of the calculation of all active schedules and possible
//! local limits present in the Charge Point. Local Limits might be taken into account.
//!
//! # Response
//! Upon receipt of a GetCompositeSchedule.req, the Charge Point SHALL calculate the Composite Charging Schedule intervals, from the moment
//! the request PDU is received: Time X, up to X + Duration, and send them in the GetCompositeSchedule.conf PDU to the Central System.
//!
//! If the ConnectorId in the request is set to '0', the Charge Point SHALL report the total expected power or current the Charge Point
//! expects to consume from the grid during the requested time period.
//!
//! Please note that the charging schedule sent by the charge point is only indicative for that pointin time. this schedule might change
//! over time due to external causes (for instance, local balancing based on grid connection capacity is active and one Connector becomes available).
//!
//! If the Charge Point is not able to report the requested schedule, for instance if the connectorId is unknown, it SHALL respond with a status Rejected
//!

pub use crate::common_types::{ChargingRateUnit, SimpleStatus};
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
