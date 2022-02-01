use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
use ocpp_json_validate::json_validate;
#[json_validate("../json_schemas/Core/Heartbeat.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Core/Heartbeat.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeartBeatResponse {
    pub current_time: DateTime<Utc>,
}
