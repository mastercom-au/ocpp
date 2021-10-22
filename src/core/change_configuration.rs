use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfigurationRequest {
    pub key: String,
    pub value: String,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfigurationResponse {
    pub status: CCoStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum CCoStatus {
    Accepted,
    Rejected,
    RebootRequired,
    NotSupported,
}
