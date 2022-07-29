//! Server request to trigger a message response from a charge point
//!
//! During normal operation, the Charge Point informs the Central System of its state and any relevant occurrences.If there is
//! nothing to report the Charge Point will send at least a heartBeat at a predefined interval. Under normal circumstances this
//! is just fine, but what if the Central System has (whatever) reason to doubt the last known state? What can a Central System
//! do if a firmware update is in progress and the last status notification it received about it was much longer ago than could
//! reasonably be expected? The same can be asked for the progress of a diagnostics request. The problem in these situations is
//! not that the information needed isn’t covered by existing messages, the problem is strictly a timing issue. The Charge Point
//! has the information, but has no way of knowing that the Central System would like an update.
//!
//! The TriggerMessage.req makes it possible for the Central System, to request the Charge Point, to send Charge Point-initiated
//! messages. In the request the Central System indicates which message it wishes to receive. For every such requested message
//! the Central System MAY optionally indicate to which connector this request applies. The requested message is leading: if the
//! specified connectorId is not relevant to the message, it should be ignored. In such cases the requested message should still
//! be sent.
//!
//! Inversely, if the connectorId is relevant but absent, this should be interpreted as “for all allowed connectorId values”.
//! For example, a request for a statusNotification for connectorId 0 is a request for the status of the Charge Point. A request
//! for a statusNotification without connectorId is a request for multiple statusNotifications: the notification for the Charge
//! Point itself and a notification for each of its connectors.

use crate::macros::{self, json_validate};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/TriggerMessage.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// This contains the field definition of the TriggerMessage.req PDU sent by the Central System to the Charge Point.
pub struct TriggerMessageRequest {
    /// Required.
    pub requested_message: MessageTrigger,
    /// Optional. Only filled in when request applies to a specific connector.
    pub connector_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
#[allow(missing_docs)]
/// Type of request to be triggered in a TriggerMessage.req.
pub enum MessageTrigger {
    BootNotification,
    DiagnosticsStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    StatusNotification,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/TriggerMessageResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// This contains the field definition of the TriggerMessage.conf PDU sent by the Charge Point to the Central System in response to a TriggerMessage.req PDU.
pub struct TriggerMessageResponse {
    /// Required. Indicates whether the Charge Point will send the requested notification or not.
    pub status: TriggerMessageStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Status in TriggerMessage.conf.
pub enum TriggerMessageStatus {
    /// Requested notification will be sent.
    Accepted,
    /// Requested notification will not be sent.
    Rejected,
    /// Requested notification cannot be sent because it is either not implemented or unknown.
    NotImplemented,
}
