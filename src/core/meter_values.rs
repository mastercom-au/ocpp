use crate::common_types::SampledValue;
use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

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
#[json_validate("../json_schemas/Core/MeterValues.json")]
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
// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Core/MeterValues.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {}
