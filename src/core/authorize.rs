use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    pub id_tag: String,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    pub id_tag_info: AuthIdTagInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthIdTagInfo {
    pub expiry_date: Option<DateTime<Utc>>,
    pub parent_id_tag: Option<String>,
    pub status: AuthStatus,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum AuthStatus {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}
