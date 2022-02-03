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

use crate::common_types::IdTagInfo;
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
    /// Required. In case of a full update this is the version number of the full list. In case of a differential update
    /// it is the version number of the list after the update has been applied.
    pub list_version: u32,
    /// Optional. In case of a full update this contains the list of values that form the new local authorization list.
    /// In case of a differential update it contains the changes to be applied to the local authorization list in the
    /// Charge Point. Maximum number of AuthorizationData elements is available in the configuration key: SendLocalListMaxLength
    pub local_authorization_list: Option<Vec<LocalAuthorizationList>>,
    /// Required. This contains the type of update (full or differential) of this request.
    pub update_type: SendLocalListUpdateType,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalAuthorizationList {
    /// Required. The identifier to which this authorization applies.
    pub id_tag: String,
    /// Optional. (Required when UpdateType is Full) This contains information about authorization status, expiry and parent id.
    /// For a Differential update the following applies: If this element is present, then this entry SHALL be added or updated in
    /// the Local Authorization List. If this element is absent, than the entry for this idtag in the Local Authorization List SHALL be deleted.
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SendLocalListUpdateType {
    /// Indicates that the current Local Authorization List must be updated with the values in this message.
    Differential,
    /// Indicates that the current Local Authorization List must be replaced by the values in this message.
    Full,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/SendLocalListResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    /// Required. This indicates whether the Charge Point has successfully received and applied the update of the local authorization list.
    pub status: UpdateStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum UpdateStatus {
    /// Local Authorization List successfully updated.
    Accepted,
    /// Failed to update the Local Authorization List.
    Failed,
    /// Update of Local Authorization List is not supported by Charge Point.
    NotSupported,
    /// Version number in the request for a differential update is less or equal then version number of current list.
    VersionMismatch,
}
