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

pub use crate::common_types::SimpleStatus;
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/RemoteStopTransaction.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionRequest {
    pub transaction_id: u32,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/RemoteStopTransactionResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionResponse {
    pub status: SimpleStatus,
}
