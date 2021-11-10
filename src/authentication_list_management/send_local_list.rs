use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

/*Structure
listVersion u32
localAuthorizationList vec<obj>
    idTag String
    updateType enum String
    idTagInfo obj
        expiryDate dateTime<utc>
        parentIdTag String
        status enum String

*/

// -------------------------- REQUEST --------------------------
#[json_validate("../json_schemas/Requests/AuthenticationListManagement/SendLocalList.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendlocalListRequest {
    pub list_version: u32,
    pub update_type: SendLocalListUpdateType,
    pub local_authorization_list: Vec<LocalAuthorizationList>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalAuthorizationList {
    pub id_tag: String,
    pub id_tag_info: LocalListIdTagInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalListIdTagInfo {
    pub expiry_date: DateTime<Utc>,
    pub parent_id_tag: String,
    pub status: SendLocalListRequestStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SendLocalListUpdateType {
    Differential,
    Full,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SendLocalListRequestStatus {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/AuthenticationListManagement/SendLocalList.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    pub status: SendLocalListResponseStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SendLocalListResponseStatus {
    Accepted,
    Failed,
    NotSupported,
    VersionMismatch,
}
