use crate::common_types::SimpleStatus;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
use ocpp_json_validate::json_validate;
#[json_validate("../json_schemas/Requests/Core/ClearCache.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/Core/ClearCache.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheResponse {
    pub status: SimpleStatus,
}
