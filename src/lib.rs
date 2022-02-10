//! # ocpp
//!
//! This library is intended to provide a framework for serialising and deserialising OCPP packets as rust types.
//! The original OCPP protocol is an [Open Charge Alliance ](https://www.openchargealliance.org/) project
//!
#![warn(missing_docs)]

pub mod common;
pub mod point_init;
pub mod server_init;

#[cfg(test)]
pub mod test;



use strum_macros::Display;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

pub use common::*;
pub use point_init::*;
pub use server_init::*;



/// Overarching OCPP Message
///
/// # Example
///
/// Decoding a heartbeat request and then encoding a heartbeat response
///
/// ```
/// # fn ocpp_message_example() -> Result<(), serde_json::Error> {
/// use ocpp::*;
/// use chrono::Utc;
///
/// // A simple request message
/// let json = r#"["2", "123", "Heartbeat", {}]"#;
///
/// // Decode generic OCPP message
/// let message: OCPPMessage = serde_json::from_str(json)?;
///
/// // Match message type
/// match message {
///     OCPPMessage::Call(call) => {
///         let response = match call.payload => {
///             // A heartbeat request
///             OCPPCallPayload::Heartbeat(_req) => {
///                 // Build response based on request
///                 OCPPMessage::CallResult(OCPPCallResult {
///                     message_type_id: 3,
///                     unique_id: call.unique_id,
///                     payload: OCPPCallResultPayload::Heartbeat(HeartbeatResponse {
///                         current_time: Utc::now();
///                     });
///                 })
///             },
///             _ => {}
///         };
///
///         // Encode response and print it
///         println!("Response: {:#?}", response.to_string());
///     },
///     _ => {}
/// }
/// # return Ok(());
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Display, Clone)]
#[serde(untagged)]
pub enum OCPPMessage {
    /// OCPP Call or Request, sent from Client to Server
    Call(OCPPCall),
    /// OCPP Call Result or Response, sent from Server to Client
    CallResult(OCPPCallResult),
    /// OCPP Call Error, sent from Server to Client
    CallError(OCPPCallError),
}

/// OCPP Call or Request, sent from Client to Server
#[derive(Serialize_tuple, Deserialize_tuple, Debug, Display, Clone)]
#[serde(tag = "action", content = "payload")]
pub struct OCPPCall {
    /// Message type ID, should always be 2
    message_type_id: u8,
    /// Unique message ID, used to match pairs of requests/responses
    pub unique_id: String,
    /// The type of message
    action: String,
    /// Message payload
    pub payload: OCPPCallPayload,
}

/// OCPP Call Result or Response, sent from Server to Client
#[derive(Serialize_tuple, Deserialize_tuple, Debug, Display, Clone)]
pub struct OCPPCallResult {
    /// Message type ID, should always be 3
    message_type_id: u8,
    /// Unique message ID, used to match pairs of requests/responses
    pub unique_id: String,
    /// Message payload
    pub payload: OCPPCallResultPayload,
}

/// OCPP Call Error, sent from Server to Client
#[derive(Serialize_tuple, Deserialize_tuple, Debug, Display, Clone)]
pub struct OCPPCallError {
    /// Message type ID, should always be 4
    message_type_id: u8,
    /// Unique message ID, used to match pairs of requests/responses
    pub unique_id: String,
    /// Short error code
    pub error_code: String,
    /// Human readable error description
    pub error_description: String,
    /// Error details, not decoded at this point
    error_details: IgnoredAny,
}

/// OCPP Call Types
#[derive(Serialize, Deserialize, Debug, PartialEq, Display, Clone)]
pub enum OCPPCallPayload {
    Authorize(AuthorizeRequest),
    BootNotification(BootNotificationRequest),
    ChangeAvailability(ChangeAvailabilityRequest),
    ChangeConfiguration(ChangeConfigurationRequest),
    ClearCache(ClearCacheRequest),
    ClearChargingProfile(ClearChargingProfileRequest),
    DataTransfer(DataTransferRequest),
    DiagnosticsStatusNotification(DiagnosticsStatusNotificationRequest),
    FirmwareStatusNotitication(FirmwareStatusNotiticationRequest),
    GetCompositeSchedule(GetCompositeScheduleRequest),
    GetConfiguration(GetConfigurationRequest),
    GetDiagnostics(GetDiagnosticsRequest),
    GetLocalListVersion(GetLocalListVersionRequest),
    Heartbeat(HeartbeatRequest),
    MeterValues(MeterValuesRequest),
    RemoteStartTransaction(RemoteStartTransactionRequest),
    RemoteStopTransacation(RemoteStopTransacationRequest),
    Reset(ResetRequest),
    SendLocalList(SendLocalListRequest),
    SetChargingProfile(SetChargingProfileRequest),
    StartTransaction(StartTransactionRequest),
    StatusNotification(StatusNotificationRequest),
    StopTransaction(StopTransactionRequest),
    TriggerMessage(TriggerMessageRequest),
    UnlockConnector(UnlockConnectorRequest),
    UpdateFirmware(UpdateFirmwareRequest),
}

/// OCPP Call Result Types
#[derive(Serialize, Deserialize, Debug, PartialEq, Display, Clone)]
pub enum OCPPCallResultPayload {
    Authorize(AuthorizeResponse),
    BootNotification(BootNotificationResponse),
    ChangeAvailability(ChangeAvailabilityResponse),
    ChangeConfiguration(ChangeConfigurationResponse),
    ClearCache(ClearCacheResponse),
    ClearChargingProfile(ClearChargingProfileResponse),
    DataTransfer(DataTransferResponse),
    DiagnosticsStatusNotification(DiagnosticsStatusNotificationResponse),
    FirmwareStatusNotitication(FirmwareStatusNotiticationResponse),
    GetCompositeSchedule(GetCompositeScheduleResponse),
    GetConfiguration(GetConfigurationResponse),
    GetDiagnostics(GetDiagnosticsResponse),
    GetLocalListVersion(GetLocalListVersionResponse),
    Heartbeat(HeartbeatResponse),
    MeterValues(MeterValuesResponse),
    RemoteStartTransaction(RemoteStartTransactionResponse),
    RemoteStopTransacation(RemoteStopTransacationResponse),
    Reset(ResetResponse),
    SendLocalList(SendLocalListResponse),
    SetChargingProfile(SetChargingProfileResponse),
    StartTransaction(StartTransactionResponse),
    StatusNotification(StatusNotificationResponse),
    StopTransaction(StopTransactionResponse),
    TriggerMessage(TriggerMessageResponse),
    UnlockConnector(UnlockConnectorResponse),
    UpdateFirmware(UpdateFirmwareResponse),
}
