//! Server request for a ChargePoint to send it's local auth list
//!
//! # Behaviour
//! Central System can send a Local Authorization List that a Charge Point can use for authorization of idTags. The list MAY be
//! either a full list to replace the current list in the Charge Point or it MAY be a differential list with updates to be applied
//! to the current list in the Charge Point.
//!
//! The Central System SHALL send a SendLocalList.req PDU to send the list to a Charge Point. The SendLocalList.req PDU SHALL contain
//! the type of update (full or differential) and the version number that the Charge Point MUST associate with the local authorization
//! list after it has been updated.
//!
//! # Response
//! Upon receipt of a SendLocalList.req PDU, the Charge Point SHALL respond with a SendLocalList.conf PDU. The response PDU SHALL
//! indicate whether the Charge Point has accepted the update of the local authorization list. If the status is Failed or VersionMismatch
//! and the updateType was Differential, then Central System SHOULD retry sending the full local authorization list with updateType Full.

use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

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
#[json_validate("../json_schemas/SendLocalList.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendlocalListRequest {
    pub list_version: u32,
    pub update_type: SendLocalListUpdateType,
    pub local_authorization_list: Option<Vec<LocalAuthorizationList>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalAuthorizationList {
    pub id_tag: String,
    pub id_tag_info: Option<LocalListIdTagInfo>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalListIdTagInfo {
    pub expiry_date: Option<DateTime<Utc>>,
    pub parent_id_tag: Option<String>,
    pub status: SendLocalListRequestStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SendLocalListUpdateType {
    Differential,
    Full,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SendLocalListRequestStatus {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/SendLocalListResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    pub status: SendLocalListResponseStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SendLocalListResponseStatus {
    Accepted,
    Failed,
    NotSupported,
    VersionMismatch,
}
