use serde::{Deserialize, Serialize};

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
