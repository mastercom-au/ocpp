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

pub use crate::common_types::{ChargingRateUnit, ChargingSchedule, SimpleStatus};
use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/GetCompositeSchedule.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the GetCompositeSchedule.req PDU sent by the Central System to the Charge Point.
pub struct GetCompositeScheduleRequest {
    /// Required. The ID of the Connector for which the schedule is requested. When ConnectorId=0, the Charge Point will calculate the expected consumption for the grid connection.
    connector_id: u32,
    /// Required. Time in seconds. length of requested schedule
    duration: u32,
    /// Optional. Can be used to force a power or current profile
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the GetCompositeSchedule.conf PDU sent by the Charge Point to the Central System in response to a GetCompositeSchedule.req PDU.
pub struct GetCompositeScheduleResponse {
    /// Required. Status of the request. The Charge Point will indicate if it was able to process the request
    pub status: SimpleStatus,
    /// Required. Status of the request. The Charge Point will indicate if it was able to process the request
    pub connector_id: Option<u32>,
    /// Optional. Time. Periods contained in the charging profile are relative to this point in time. If status is "Rejected", this field may be absent.
    pub schedule_start: DateTime<Utc>,
    /// Optional. Planned Composite Charging Schedule, the energy consumption over time. Always relative to ScheduleStart. If status is "Rejected", this field may be absent.
    pub charging_schedule: Option<ChargingSchedule>,
}
