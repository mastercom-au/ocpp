//! Request from chargepoint to end an energy transaction. *Central server cannot deny this request.*
//!
//! # Behaviour
//! A StopTransaction.req PDU MAY contain an optional TransactionData element to provide more details about transaction usage. The optional TransactionData element
//! is a container for any number of MeterValues, using the same data structure as the meterValue elements of the MeterValues.req PDU (See section MeterValues)
//!
//! The idTag in the request PDU MAY be omitted when the Charge Point itself needs to stop the transaction. For instance, when the Charge Point is requested to reset.
//!
//! If a transaction is ended in a normal way (e.g. EV-driver presented his identification to stop the transaction), the Reason element MAY be omitted and the Reason
//! SHOULD be assumed 'Local'. If the transaction is not ended normally, the Reason SHOULD be set to a correct value. As part of the normal transaction termination,
//! the Charge Point SHALL unlock the cable (if not permanently attached).
//!
//! The Charge Point MAY unlock the cable (if not permanently attached) when the cable is disconnected at the EV. If supported, this functionality is reported and
//! controlled by the configuration key UnlockConnectorOnEVSideDisconnect.
//!
//! The Charge Point MAY stop a running transaction when the cable is disconnected at the EV. If supported, this functionality is reported and controlled by the
//! configuration key StopTransactionOnEVSideDisconnect.
//!
//! # Response
//! Upon receipt of a StopTransaction.req PDU, the Central System SHALL respond with a StopTransaction.conf PDU. The Central System cannot prevent a transaction from
//! stopping. It MAY only inform the Charge Point it has received the StopTransaction.req and MAY send information about the idTag used to stop the transaction. This
//! information SHOULD be used to update the Authorization Cache, if implemented.
//!
//! # StopTransactionOnEVSideDisconnect
//!
//! If StopTransactionOnEVSideDisconnect is set to false, the transaction SHALL not be stopped when the cable is disconnected from the EV. If the EV is reconnected,
//! energy transfer is allowed again. In this case there is no mechanism to prevent other EVs from charging and disconnecting during that same ongoing transaction.
//! With UnlockConnectorOnEVSideDisconnect set to false, the Connector SHALL remain locked at the Charge Point until the user presents the identifier.
//!
//! By setting StopTransactionOnEVSideDisconnect to true, the transaction SHALL be stopped when the cable is disconnected from the EV. If the EV is reconnected, energy
//! transfer is not allowed until the transaction is stopped and a new transaction is started. If UnlockConnectorOnEVSideDisconnect is set to true, also the Connector
//! on the Charge Point will be unlocked. If StopTransactionOnEVSideDisconnect is set to false, this SHALL have priority over UnlockConnectorOnEVSideDisconnect. In other words: cables always remain locked
//! when the cable is disconnected at EV side when StopTransactionOnEVSideDisconnect is false. Setting StopTransactionOnEVSideDisconnect to true will prevent sabotage acts to stop the energy flow by unplugging not locked cables on EV side.
//!
//! It is likely that The Central System applies sanity checks to the data contained in a StopTransaction.req it received. The outcome of such sanity checks SHOULD
//! NOT ever cause the Central System to not respond with a StopTransaction.conf. Failing to respond with a StopTransaction.conf will only cause the Charge Point
//! to try the same message again as specified in Error responses to transaction-related messages.
//!
//! If Charge Point has implemented an Authorization Cache, then upon receipt of a StopTransaction.conf PDU the Charge Point SHALL update the cache entry, if the
//! idTag is not in the Local Authorization List, with the IdTagInfo value from the response as described under Authorization Cache.

use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

pub use crate::common_types::IdTagInfo;
pub use crate::MeterValue;

/*Structure
idTag String
meterStop u32
timestamp dateTime<utc>
transactionId u32,
reason enum String,
transactionData vec<obj>
    timeStamp dateTime<utc>
    sampledValue vec<obj>
        value string
        context enum String
        format enum String
        measurand enum String
        phase enum String
        location enum String
        unit enum String
*/

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/StopTransaction.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the StopTransaction.req PDU sent by the Charge Point to the Central System.
pub struct StopTransactionRequest {
    /// Optional. This contains the identifier which requested to stop the charging. It is optional because a Charge
    /// Point may terminate charging without the presence of an idTag, e.g. in case of a reset. A Charge Point SHALL send the idTag if known.
    pub id_tag: Option<String>,
    /// Required. This contains the meter value in Wh for the connector at end of the transaction.
    pub meter_stop: u32,
    /// Required. This contains the date and time on which the transaction is stopped.
    pub timestamp: DateTime<Utc>,
    /// Required. This contains the transaction-id as received by the StartTransaction.conf.
    pub transaction_id: u32,
    /// Optional. This contains the reason why the transaction was stopped. MAY only be omitted when the Reason is "Local".
    pub reason: Option<StopReason>,
    /// Optional. This contains transaction usage details relevant for billing purposes.
    pub transaction_data: Option<Vec<MeterValue>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Reason for stopping a transaction in StopTransaction.req.
pub enum StopReason {
    /// Emergency stop button was used.
    EmergencyStop,
    /// disconnecting of cable, vehicle moved away from inductive charge unit.
    EVDisconnected,
    /// A hard reset command was received.
    HardReset,
    /// Stopped locally on request of the user at the Charge Point. This is a regular termination of a transaction. Examples: presenting an RFID tag, pressing a button to stop.
    Local,
    /// Any other reason.
    Other,
    /// Complete loss of power.
    PowerLoss,
    /// A locally initiated reset/reboot occurred. (for instance watchdog kicked in)
    Reboot,
    /// Stopped remotely on request of the user. This is a regular termination of a transaction. Examples: termination using a smartphone app, exceeding a (non local) prepaid credit.
    Remote,
    /// A soft reset command was received.
    SoftReset,
    /// Central System sent an Unlock Connector command.
    UnlockCommand,
    ///
    DeAuthorized,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/StopTransactionResponse.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the StopTransaction.conf PDU sent by the Central System to the Charge Point in response to a StopTransaction.req PDU.
pub struct StopTransactionResponse {
    /// see [idtaginfo](crate::common_types::IdTagInfo)
    pub id_tag_info: Option<IdTagInfo>,
}
