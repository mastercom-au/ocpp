use crate::common_types::SimpleStatus;
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/Core/Reset.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    pub r#type: ResetType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ResetType {
    Hard,
    Soft,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Core/ResetResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    status: SimpleStatus,
}
