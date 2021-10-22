use serde::{Deserialize, Serialize};

// -------------------------- REQUEST --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    pub list_version: u32,
}