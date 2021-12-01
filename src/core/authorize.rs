use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
// -------------------------- REQUEST --------------------------
#[json_validate("../json_schemas/Requests/Core/Authorize.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    pub id_tag: String,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/Core/Authorize.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    pub id_tag_info: AuthIdTagInfo,
}

#[skip_serializing_none]
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
