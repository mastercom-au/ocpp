use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {}

pub struct ClearCacheResponse {
    pub status: CCaStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum CCaStatus {
    Accepted,
    Rejected,
}
