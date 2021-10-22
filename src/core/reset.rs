use crate::common_types::SimpleStatus;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    r#type: ResetType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ResetType {
    Hard,
    Soft,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    status: SimpleStatus,
}
