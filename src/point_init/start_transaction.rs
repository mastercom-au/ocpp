//! Request from chargepoint to begin an energy transaction. Must be sent before charging will begin.
//!
//! # Behaviour
//! If this transaction ends a reservation (see Reserve Now operation), then the StartTransaction.req MUST contain the reservationId.
//! If Charge Point has implemented an Authorization Cache, then upon receipt of a StartTransaction.conf PDU the Charge Point SHALL update the cache
//! entry, if the idTag is not in the Local Authorization List, with the IdTagInfo value from the response as described under Authorization Cache.
//!  
//! # Response
//! Upon receipt of a StartTransaction.req PDU, the Central System SHOULD respond with a StartTransaction.conf PDU. This response PDU MUST include a transaction
//! id and an authorization status value. The Central System MUST verify validity of the identifier in the StartTransaction.req PDU, because the
//! identifier might have been authorized locally by the Charge Point using outdated information. The identifier, for instance, may have been blocked
//! since it was added to the Charge Point’s Authorization Cache.
//!
//! It is likely that The Central System applies sanity checks to the data contained in a StartTransaction.req it received. The outcome of such sanity
//! checks SHOULD NOT ever cause the Central System to not respond with a StartTransaction.conf. Failing to respond with a StartTransaction.conf will
//! only cause the Charge Point to try the same message again as specified in Error responses to transaction-related messages.

pub use crate::common_types::IdTagInfo;
use crate::macros::{self, json_validate};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/StartTransaction.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the StartTransaction.req PDU sent by the Charge Point to the Central System
pub struct StartTransactionRequest {
    /// Required. This identifies which connector of the Charge Point is used.
    pub connector_id: u32,
    /// Required. This contains the identifier for which a transaction has to be started.
    pub id_tag: String,
    /// Required. This contains the meter value in Wh for the connector at start of the transaction
    pub meter_start: i32,
    /// Optional. This contains the id of the reservation that terminates as a result of this transaction.
    pub reservation_id: Option<i32>,
    /// Required. This contains the date and time on which the transaction is started.
    pub timestamp: DateTime<Utc>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/StartTransactionResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the StartTransaction.conf PDU sent by the Central System to the Charge Point in response to a StartTransaction.req PDU.
pub struct StartTransactionResponse {
    /// Required. This contains information about authorization status, expiry and parent id
    pub id_tag_info: IdTagInfo,
    /// Required. This contains the transaction id supplied by the Central System.
    pub transaction_id: u32,
}
