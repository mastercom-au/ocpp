//! Server request to stop a transaction
//!
//! # Behaviour
//! Central System can request a Charge Point to stop a transaction by sending a RemoteStopTransaction.req to Charge Point with the
//! identifier of the transaction.
//!
//! This remote request to stop a transaction is equal to a local action to stop a transaction. Therefore, the transaction SHALL be
//! stopped, The Charge Point SHALL send a StopTransaction.req and, if applicable, unlock the connector.
//!
//! The following two main use cases are the reason for Remote Stop Transaction:
//! * Enable a CPO operator to help an EV driver that has problems stopping a transaction.
//! * Enable mobile apps to control charging transactions via the Central System.
//!
//! # Response
//! Charge Point SHALL reply with RemoteStopTransaction.conf and a status indicating whether it has
//! accepted the request and a transaction with the given transactionId is ongoing and will be stopped.
//!

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

pub use crate::common_types::SimpleStatus;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/RemoteStopTransaction.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definitions of the RemoteStopTransaction.req PDU sent to Charge Point by Central System.
pub struct RemoteStopTransactionRequest {
    /// Required. The identifier of the transaction which Charge Point is requested to stop
    pub transaction_id: u32,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/RemoteStopTransactionResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definitions of the RemoteStopTransaction.conf PDU sent from Charge Point to Central System.
pub struct RemoteStopTransactionResponse {
    /// Required. Status indicating whether Charge Point accepts the request to stop a transaction.
    pub status: SimpleStatus,
}
