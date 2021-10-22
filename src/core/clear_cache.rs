use crate::common_types::SimpleStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {}

pub struct ClearCacheResponse {
    pub status: SimpleStatus,
}
