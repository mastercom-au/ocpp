//! Authorization message sent before transaction start/end. *Not required in free vend mode*
//!
//! # Behaviour
//! Before the owner of an electric vehicle can start or stop charging, the Charge Point has to authorize the
//! operation. The Charge Point SHALL only supply energy after authorization. When stopping a Transaction, the
//! Charge Point SHALL only send an Authorize.req when the identifier used for stopping the transaction is different
//! from the identifier that started the transaction.
//!
//! Authorize.req SHOULD only be used for the authorization of an identifier for charging.
//! A Charge Point MAY authorize identifier locally without involving the Central System, as described in Local
//! Authorization List. If an idTag presented by the user is not present in the Local Authorization List or
//! Authorization Cache, then the Charge Point SHALL send an Authorize.req PDU to the Central System to request
//! authorization. If the idTag is present in the Local Authorization List or Authorization Cache, then the Charge Point
//! MAY send an Authorize.req PDU to the Central System.
//!
//! If Charge Point has implemented an Authorization Cache, then upon receipt of an Authorize.conf PDU the
//! Charge Point SHALL update the cache entry, if the idTag is not in the Local Authorization List, with the IdTagInfo
//!
//!  # Response
//! Upon receipt of an Authorize.req PDU, the Central System SHALL respond with an Authorize.conf PDU. This
//! response PDU SHALL indicate whether or not the idTag is accepted by the Central System. If the Central System
//! accepts the idTag then the response PDU MAY include a parentIdTag and MUST include an authorization status
//! value indicating acceptance or a reason for rejection.
//!

pub use crate::common::common_types::IdTagInfo;
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST --------------------------
#[json_validate("../json_schemas/Authorize.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the Authorize.req PDU sent by the Charge Point to the Central System.
pub struct AuthorizeRequest {
    ///This contains the identifier that needs to be authorized.
    pub id_tag: String,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/AuthorizeResponse.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the Authorize.conf PDU sent by the Central System to the Charge Point in response to a Authorize.req PDU.
pub struct AuthorizeResponse {
    ///This contains information about authorization status, expiry and parent id.
    pub id_tag_info: IdTagInfo,
}
