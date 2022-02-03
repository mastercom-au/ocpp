//! Configurable ping to verify charge point is still connected.
//! # Behaviour
//! The Charge Point SHALL send a Heartbeat.req PDU for ensuring that the Central System knows that a Charge Point is still alive.
//! Upon receipt of a Heartbeat.req PDU, the Central System SHALL respond with a Heartbeat.conf.
//!
//! # Response
//! The response PDU SHALL contain the current time of the Central System, which is RECOMMENDED to be used by the Charge Point to synchronize its internal clock.
//! The Charge Point MAY skip sending a Heartbeat.req PDU when another PDU has been sent to the Central System within the configured heartbeat interval. This implies that a Central System SHOULD assume availability
//! of a Charge Point whenever a PDU has been received, the same way as it would have, when it received a Heartbeat.req PDU.
//!
//! With JSON over WebSocket, sending heartbeats is not mandatory. However, for time synchronization it is advised to at least send one heartbeat per 24 hour.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
use ocpp_json_validate::json_validate;
#[json_validate("../json_schemas/Heartbeat.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/HeartbeatResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeartBeatResponse {
    pub current_time: DateTime<Utc>,
}
