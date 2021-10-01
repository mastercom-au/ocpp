use chrono::{DateTime, Utc};
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendlocalListRequest {
    pub list_version: u32,
    pub update_type: UpdateType,
    pub local_authorization_list: Vec<LocalAuthorizationList>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalAuthorizationList {
    pub id_tag: String,
    pub id_tag_info: IdTagInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    pub expiry_date: DateTime<Utc>,
    pub parent_id_tag: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum UpdateType {
    Differential,
    Full,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}
