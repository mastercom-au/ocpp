use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::common_types::{
    SampledFormat, SampledContext, SampledMeasurand, SampledPhase, SampledLocation, SampledUnit};

/* Structure
connectorId u32
transactonId u32
meterValue Vec<obj>
    timestamp dateTime<utc>
    sampledValue Vec<obj>
        value String
        context enum String
        format enum String
        measurand enum String
        phase enum String
        location enum String
        unit enum String
*/

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    pub connector_id: u32,
    pub transaction_id: Option<u32>,
    pub meter_value: Vec<MeterValues>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeterValues {
    pub timestamp: DateTime<Utc>,
    pub sampled_value: Vec<SampledValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    pub value: String,
    pub context: Option<SampledContext>,
    pub format: Option<SampledFormat>,
    pub measurant: Option<SampledMeasurand>,
    pub phase: Option<SampledPhase>,
    pub location: Option<SampledLocation>,
    pub unit: Option<SampledUnit>,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {}
