//! Server request for a ChargePoint to clear it's auth cache.
//!
//! # Response
//! Upon receipt of a ClearCache.req PDU, the Charge Point SHALL respond with a ClearCache.conf PDU. The response PDU
//! SHALL indicate whether the Charge Point was able to clear its Authorization Cache.

pub use crate::common_types::SimpleStatus;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
use ocpp_json_validate::json_validate;
#[json_validate("../json_schemas/ClearCache.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// This contains the field definition of the ClearCache.req PDU sent by the Central System to the Charge Point.
pub struct ClearCacheRequest {}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/ClearCacheResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// This contains the field definition of the ClearCache.conf PDU sent by the Charge Point to the Central System in Response to a ClearCache.
pub struct ClearCacheResponse {
    /// Required. Accepted if the Charge Point has executed the request, otherwise rejected.
    pub status: SimpleStatus,
}
