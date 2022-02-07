//! A notification is sent when a connector changes status or the chargepoint has an error.
//!
//! # Behaviour
//! When a Charge Point connects to a Central System after having been offline, it updates the Central System about its status according to the following rules:
//! 1. The Charge Point SHOULD send a StatusNotification.req PDU with its current status if the status changed while the Charge Point was offline.
//! 2. The Charge Point MAY send a StatusNotification.req PDU to report an error that occurred while the Charge Point was offline.
//! 3. The Charge Point SHOULD NOT send StatusNotification.req PDUs for historical status change events that happened while the Charge Point was offline
//! and that do not inform the Central System of Charge Point errors or the Charge Point’s current status.
//! 4. The StatusNotification.req messages MUST be sent in the order in which the events that they describe occurred.
//!
//! To limit the number of transitions, the Charge Point MAY omit sending a StatusNotification.req if it was active for
//! less time than defined in the optional configuration key MinimumStatusDuration. This way, a Charge Point MAY choose
//! not to send certain StatusNotification.req PDUs.
//!
//! A Charge Point manufacturer MAY have implemented a minimal status duration for certain status transitions separate of the
//! MinimumStatusDuration setting. The time set in MinimumStatusDuration will be added to this default delay. Setting MinimumStatusDuration
//! to zero SHALL NOT override the default manufacturer’s minimal status duration.
//!
//! The Charge Point MAY send a StatusNotification.req PDU to inform the Central System of fault conditions. When the 'status' field is not
//! Faulted, the condition should be considered a warning since charging operations are still possible.
//!
//! # StopTransactionOnEvSideDisconnect
//! When a Charge Point is configured with StopTransactionOnEVSideDisconnect set to false, a transaction is running and the EV becomes disconnected on
//! EV side, then a StatusNotification.req with the state: SuspendedEV SHOULD be send to the Central System, with the 'errorCode' field set to: 'NoError'.
//! The Charge Point SHOULD add additional information in the 'info' field, Notifying the Central System with the reason of suspension: 'EV side disconnected'. The current transaction is not stopped.
//!
//! When a Charge Point is configured with StopTransactionOnEVSideDisconnect set to true, a transaction is running and the EV becomes disconnected on
//! EV side, then a StatusNotification.req with the state: 'Finishing' SHOULD be send to the Central System, with the 'errorCode' field set to: 'NoError'.
//! The Charge Point SHOULD add additional information in the 'info' field, Notifying the Central System with the reason of stopping: 'EV side disconnected'. The current transaction is stopped.
//!
//! # Response
//! Upon receipt of a StatusNotification.req PDU, the Central System SHALL respond with a StatusNotification.conf PDU.

use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/StatusNotification.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the StatusNotification.req PDU sent by the Charge Point to the Central System.
pub struct StatusNotificationRequest {
    /// Required. The id of the connector for which the status is reported. Id '0' (zero) is used if the status is for the Charge Point main controller
    pub connector_id: u32,
    /// Required. This contains the error code reported by the Charge Point.
    pub error_code: StatusNotificationErrorCode,
    /// Optional. Additional free format information related to the error.
    pub info: Option<String>,
    /// Required. This contains the current status of the Charge Point.
    pub status: StatusNotificationStatus,
    /// Optional. The time for which the status is reported. If absent time of receipt of the message will be assumed.
    pub timestamp: Option<DateTime<Utc>>,
    /// Optional. This identifies the vendor-specific implementation.
    pub vendor_id: Option<String>,
    /// Optional. This contains the vendor-specific error code.
    pub vendor_error_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Charge Point Error Code reported in StatusNotification.req.
pub enum StatusNotificationErrorCode {
    /// Failure to lock or unlock connector.
    ConnectorLockFailure,
    /// Communication failure with the vehicle, might be Mode 3 or other communication protocol problem. This is not a real error in the sense that
    /// the Charge Point doesn’t need to go to the faulted state. Instead, it should go to the SuspendedEVSE state.
    EVCommunicationError,
    /// Ground fault circuit interrupter has been activated.
    GroundFailure,
    /// Temperature inside Charge Point is too high.
    HighTemperature,
    /// Error in internal hard- or software component.
    InternalError,
    /// The authorization information received from the Central System is in conflict with the LocalAuthorizationList.
    LocalListConflict,
    /// No error to report.
    NoError,
    /// Other type of error. More information in vendorErrorCode.
    OtherError,
    /// Over current protection device has tripped.
    OverCurrentFailure,
    /// Failure to read electrical/energy/power meter.
    PowerMeterFailure,
    /// Failure to read electrical/energy/power meter.
    PowerSwitchFailure,
    /// Failure with idTag reader.
    ReaderFailure,
    /// Unable to perform a reset.
    ResetFailure,
    /// Voltage has dropped below an acceptable level.
    UnderVoltage,
    /// Voltage has risen above an acceptable level.
    OverVoltage,
    /// Wireless communication device reports a weak signal.
    WeakSignal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Status reported in StatusNotification.req.
///
/// A status can be reported for the Charge Point main controller (connectorId = 0) or for a specific connector. Status for the Charge Point main controller is a subset of the enumeration: Available, Unavailable or Faulted.
///
/// States considered Operative are: Available, Preparing, Charging, SuspendedEVSE, SuspendedEV, Finishing, Reserved.
///
/// States considered Inoperative are: Unavailable, Faulted.
pub enum StatusNotificationStatus {
    /// When a Connector becomes available for a new user (Operative)
    Available,
    /// When a Connector becomes no longer available for a new user but there is no ongoing Transaction (yet). Typically a Connector is in
    /// preparing state when a user presents a tag, inserts a cable or a vehicle occupies the parking bay
    Preparing,
    /// When the contactor of a Connector closes, allowing the vehicle to charge
    Charging,
    /// When the EV is connected to the EVSE but the EVSE is not offering energy to the EV, e.g. due to a smart charging restriction, local
    /// supply power constraints, or as the result of StartTransaction.conf indicating that charging is not allowed etc.
    SuspendedEVSE,
    /// When the EV is connected to the EVSE and the EVSE is offering energy but the EV is not taking any energy.
    SuspendedEV,
    /// When a Transaction has stopped at a Connector, but the Connector is not yet available for a new user, e.g. the cable has not been
    /// removed or the vehicle has not left the parking bay
    Finishing,
    /// When a Connector becomes reserved as a result of a Reserve Now command
    Reserved,
    /// When a Connector becomes unavailable as the result of a Change Availability command or an event upon which the Charge Point transitions
    /// to unavailable at its discretion. Upon receipt of a Change Availability command, the status MAY change immediately or the change MAY be
    /// scheduled. When scheduled, the Status Notification shall be send when the availability change becomes effective
    Unavailable,
    /// When a Charge Point or connector has reported an error and is not available for energy delivery . (Inoperative).
    Faulted,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/StatusNotificationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the StatusNotification.conf PDU sent by the Central System to the Charge Point in response to an StatusNotification.req PDU.
pub struct StatusNotificationResponse {}
