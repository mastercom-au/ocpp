//! # ocpp
//!
//! This library is intended to provide a framework for serialising and deserialising OCPP packets as rust types.
//! The original OCPP protocol is an [Open Charge Alliance ](https://www.openchargealliance.org/) project
//!
//! # Example
//!
//! Decoding a heartbeat request and then encoding a heartbeat response
//!
//! ```
//! # fn ocpp_message_example() -> Result<(), serde_json::Error> {
//! use ocpp::*;
//!
//! // Example of storage for known/open calls
//! let mut known_calls = std::collections::HashMap::<String, OCPPCallAction>::new();
//!
//! // Build a reusable response builder
//! // Method 1. Define a method
//! fn handle_authorize(req: AuthorizeRequest) -> Result<AuthorizeResponse, OCPPCallErrorCode>
//! {
//!     let status = if req.id_tag == "valid_tag" {
//!         AuthorizationStatus::Accepted
//!     } else {
//!         AuthorizationStatus::Blocked
//!     };
//!
//!     Ok(AuthorizeResponse {
//!         id_tag_info: IdTagInfo {
//!             expiry_date: None,
//!             parent_id_tag: None,
//!             status,
//!         }
//!     })
//! }
//!
//! let mut call_result_builder = OCPPCall::result_builder()
//!     // Method 2. Pass in a closure directly
//!     .for_heartbeat(Box::new(|_req| {
//!         Ok(
//!             HeartbeatResponse {
//!                 current_time: chrono::Utc::now()
//!             }
//!         )
//!     }))
//!     // Method 2.
//!     .for_boot_notification(Box::new(|_req| {
//!         Ok(
//!             BootNotificationResponse {
//!                 status: BootNotificationStatus::Accepted,
//!                 current_time: chrono::Utc::now(),
//!                 interval: 5,
//!             }
//!         )
//!     }))
//!     // Method 1.
//!     .for_authorize(Box::new(handle_authorize));
//!
//! // A simple request message, for demonstration
//! let json = r#"["2", "123", "Heartbeat", {}]"#;
//!
//! // Decode generic OCPP message
//! let message: OCPPMessage = serde_json::from_str(json)?;
//!
//! // Match message type
//! match message {
//!     OCPPMessage::Call(call) => {
//!         // Invoke builder to get response
//!         match call_result_builder.build(call) {
//!             Ok(res) => {
//!                 // A result was returned, encode and print it
//!                 println!("Response: {:#?}", serde_json::to_string(&res));
//!             },
//!             Err(e) => {
//!                 // Some error occured during processing
//!                 println!("Error processing request: {:#?}", e);
//!             }
//!         }
//!     },
//!     OCPPMessage::CallResult(result) => {
//!         println!("Call Result: {:#?}", result);
//!     },
//!     OCPPMessage::CallResultUnknown(unknown) => {
//!         // Lookup the call ID in known_calls
//!         if let Some(action) = known_calls.get(&unknown.unique_id) {
//!             // Convert and print known call result
//!             let result = OCPPCallResult::from_unknown(action, unknown)?;
//!             println!("Call Result: {:#?}", result);
//!         } else {
//!             println!("Unknown Call Result: {:#?}", &unknown.unique_id);
//!         }
//!     },
//!     OCPPMessage::CallError(err) => {
//!         println!("Call Error: {:#?}", err);
//!     }
//! }
//! # return Ok(());
//! # }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;

pub mod common;
pub mod point_init;
pub mod server_init;

#[cfg(test)]
pub mod test;



use std::boxed::Box;
use std::default::Default;

use strum_macros::Display;
use serde::{de, Deserialize, Serialize, Deserializer, Serializer};

use ocpp_json_validate::JsonValidate;

pub use common::*;
pub use point_init::*;
pub use server_init::*;



/// Overarching OCPP Message use to encapsulate calls, call results and call errors
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OCPPMessage {
    /// OCPP Call or Request, sent from Client to Server
    Call(OCPPCall),
    /// OCPP Call Result or Response, sent from Server to Client
    #[serde(skip_deserializing)]
    CallResult(OCPPCallResult),
    /// OCPP Call Result or Response, sent from Server to Client
    #[serde(skip_serializing)]
    CallResultUnknown(OCPPCallResultUnknown),
    /// OCPP Call Error, sent from Server to Client
    CallError(OCPPCallError),
}

/// OCPP Call or Request, sent from Client to Server
#[derive(Debug, Clone)]
pub struct OCPPCall {
    /// Unique message ID, used to match pairs of requests/responses
    pub unique_id: String,
    /// The type of message
    action: String,
    /// Message payload
    pub payload: OCPPCallPayload,
}

impl Serialize for OCPPCall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        (2, &self.unique_id, &self.action, &self.payload).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OCPPCall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let (message_type_id, unique_id, action, payload_raw): (u8, String, String, serde_json::Value) = Deserialize::deserialize(deserializer)?;

        if message_type_id != 2 {
            return Err(de::Error::invalid_value(de::Unexpected::Unsigned(message_type_id.into()), &"Message Type ID for Call should be '2'"));
        }

        let payload = match action.as_ref() {
            "Authorize" => OCPPCallPayload::Authorize(AuthorizeRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "BootNotification" => OCPPCallPayload::BootNotification(BootNotificationRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "ChangeAvailability" => OCPPCallPayload::ChangeAvailability(ChangeAvailabilityRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "ChangeConfiguration" => OCPPCallPayload::ChangeConfiguration(ChangeConfigurationRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "ClearCache" => OCPPCallPayload::ClearCache(ClearCacheRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "ClearChargingProfile" => OCPPCallPayload::ClearChargingProfile(ClearChargingProfileRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "DataTransfer" => OCPPCallPayload::DataTransfer(DataTransferRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "DiagnosticsStatusNotification" => OCPPCallPayload::DiagnosticsStatusNotification(DiagnosticsStatusNotificationRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "FirmwareStatusNotification" => OCPPCallPayload::FirmwareStatusNotification(FirmwareStatusNotificationRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "GetCompositeSchedule" => OCPPCallPayload::GetCompositeSchedule(GetCompositeScheduleRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "GetConfiguration" => OCPPCallPayload::GetConfiguration(GetConfigurationRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "GetDiagnostics" => OCPPCallPayload::GetDiagnostics(GetDiagnosticsRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "GetLocalListVersion" => OCPPCallPayload::GetLocalListVersion(GetLocalListVersionRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "Heartbeat" => OCPPCallPayload::Heartbeat(HeartbeatRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "MeterValues" => OCPPCallPayload::MeterValues(MeterValuesRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "RemoteStartTransaction" => OCPPCallPayload::RemoteStartTransaction(RemoteStartTransactionRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "RemoteStopTransaction" => OCPPCallPayload::RemoteStopTransaction(RemoteStopTransactionRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "Reset" => OCPPCallPayload::Reset(ResetRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "SendLocalList" => OCPPCallPayload::SendLocalList(SendLocalListRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "SetChargingProfile" => OCPPCallPayload::SetChargingProfile(SetChargingProfileRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "StartTransaction" => OCPPCallPayload::StartTransaction(StartTransactionRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "StatusNotification" => OCPPCallPayload::StatusNotification(StatusNotificationRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "StopTransaction" => OCPPCallPayload::StopTransaction(StopTransactionRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "TriggerMessage" => OCPPCallPayload::TriggerMessage(TriggerMessageRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "UnlockConnector" => OCPPCallPayload::UnlockConnector(UnlockConnectorRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            "UpdateFirmware" => OCPPCallPayload::UpdateFirmware(UpdateFirmwareRequest::deserialize(payload_raw).map_err(|e| de::Error::custom(format!("{}", e)))?),
            _ => {
                return Err(de::Error::unknown_variant(&action, &[
                    "Authorize",
                    "BootNotification",
                    "ChangeAvailability",
                    "ChangeConfiguration",
                    "ClearCache",
                    "ClearChargingProfile",
                    "DataTransfer",
                    "DiagnosticsStatusNotification",
                    "FirmwareStatusNotification",
                    "GetCompositeSchedule",
                    "GetConfiguration",
                    "GetDiagnostics",
                    "GetLocalListVersion",
                    "Heartbeat",
                    "MeterValues",
                    "RemoteStartTransaction",
                    "RemoteStopTransaction",
                    "Reset",
                    "SendLocalList",
                    "SetChargingProfile",
                    "StartTransaction",
                    "StatusNotification",
                    "StopTransaction",
                    "TriggerMessage",
                    "UnlockConnector",
                    "UpdateFirmware",
                ]));
            }
        };

        Ok(OCPPCall {
            unique_id,
            action,
            payload,
        })
    }
}

impl OCPPCall {
    /// Create default/empty [OCPPCallResultBuilder]. By default all methods
    /// will return `Err(OCPPCallErrorCode::NotImplemented)`
    pub fn result_builder() -> OCPPCallResultBuilder {
        Default::default()
    }
}

/// OCPP Call Result or Response, sent from Server to Client
/// For deserialization see [OCPPCallResultUnknown] and [OCPPCallResult::from_unknown]
#[derive(Debug, Clone)]
pub struct OCPPCallResult {
    /// Unique message ID, used to match pairs of requests/responses
    pub unique_id: String,
    /// Message payload
    pub payload: OCPPCallResultPayload,
}

impl Serialize for OCPPCallResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        (3, &self.unique_id, &self.payload).serialize(serializer)
    }
}

/// OCPP Call Result or Response, sent from Server to Client
/// Helper type for partial deserialization of unknown types of result
/// Use this in conjunction with [OCPPCallResult::from_unknown]
#[derive(Debug, Clone)]
pub struct OCPPCallResultUnknown {
    /// Unique message ID, used to match pairs of requests/responses
    pub unique_id: String,
    /// Message payload
    pub payload: serde_json::Value,
}

impl<'de> Deserialize<'de> for OCPPCallResultUnknown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let (message_type_id, unique_id, payload): (u8, String, serde_json::Value) = Deserialize::deserialize(deserializer)?;

        if message_type_id != 3 {
            return Err(de::Error::invalid_value(de::Unexpected::Unsigned(message_type_id.into()), &"Message Type ID for Call Result should be '3'"));
        }

        Ok(OCPPCallResultUnknown {
            unique_id,
            payload,
        })
    }
}

impl OCPPCallResult
{
    /// Convert OCPP Call result of an unspecified type into a specific and
    /// valid call result. Fails in case the provided call result is not a
    /// valid instance of the specified call type
    ///
    /// # Example
    /// ```
    /// # fn ocpp_from_unknown_example() -> Result<(), serde_json::Error> {
    /// use ocpp::*;
    ///
    /// // Decode message generically
    /// let json = "[3,\"63:2\",{}]";
    /// let value: OCPPMessage = serde_json::from_str(json)?;
    ///
    /// if let OCPPMessage::CallResultUnknown(unknown) = value {
    ///     // Convert from CallResultUnknown to CallResult, in this case assuming that we have a
    ///     // StatusNotification
    ///     let result = OCPPCallResult::from_unknown(&OCPPCallAction::StatusNotification, unknown)?;
    ///     println!("Decoded status notification response: {:#?}", result)
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_unknown(action: &OCPPCallAction, unknown: OCPPCallResultUnknown) -> Result<Self, serde_json::Error>
    {
        let OCPPCallResultUnknown {
            unique_id,
            payload
        } = unknown;

        let payload = match action {
            OCPPCallAction::Authorize => OCPPCallResultPayload::Authorize(AuthorizeResponse::deserialize(payload)?),
            OCPPCallAction::BootNotification => OCPPCallResultPayload::BootNotification(BootNotificationResponse::deserialize(payload)?),
            OCPPCallAction::ChangeAvailability => OCPPCallResultPayload::ChangeAvailability(ChangeAvailabilityResponse::deserialize(payload)?),
            OCPPCallAction::ChangeConfiguration => OCPPCallResultPayload::ChangeConfiguration(ChangeConfigurationResponse::deserialize(payload)?),
            OCPPCallAction::ClearCache => OCPPCallResultPayload::ClearCache(ClearCacheResponse::deserialize(payload)?),
            OCPPCallAction::ClearChargingProfile => OCPPCallResultPayload::ClearChargingProfile(ClearChargingProfileResponse::deserialize(payload)?),
            OCPPCallAction::DataTransfer => OCPPCallResultPayload::DataTransfer(DataTransferResponse::deserialize(payload)?),
            OCPPCallAction::DiagnosticsStatusNotification => OCPPCallResultPayload::DiagnosticsStatusNotification(DiagnosticsStatusNotificationResponse::deserialize(payload)?),
            OCPPCallAction::FirmwareStatusNotification => OCPPCallResultPayload::FirmwareStatusNotification(FirmwareStatusNotificationResponse::deserialize(payload)?),
            OCPPCallAction::GetCompositeSchedule => OCPPCallResultPayload::GetCompositeSchedule(GetCompositeScheduleResponse::deserialize(payload)?),
            OCPPCallAction::GetConfiguration => OCPPCallResultPayload::GetConfiguration(GetConfigurationResponse::deserialize(payload)?),
            OCPPCallAction::GetDiagnostics => OCPPCallResultPayload::GetDiagnostics(GetDiagnosticsResponse::deserialize(payload)?),
            OCPPCallAction::GetLocalListVersion => OCPPCallResultPayload::GetLocalListVersion(GetLocalListVersionResponse::deserialize(payload)?),
            OCPPCallAction::Heartbeat => OCPPCallResultPayload::Heartbeat(HeartbeatResponse::deserialize(payload)?),
            OCPPCallAction::MeterValues => OCPPCallResultPayload::MeterValues(MeterValuesResponse::deserialize(payload)?),
            OCPPCallAction::RemoteStartTransaction => OCPPCallResultPayload::RemoteStartTransaction(RemoteStartTransactionResponse::deserialize(payload)?),
            OCPPCallAction::RemoteStopTransaction => OCPPCallResultPayload::RemoteStopTransaction(RemoteStopTransactionResponse::deserialize(payload)?),
            OCPPCallAction::Reset => OCPPCallResultPayload::Reset(ResetResponse::deserialize(payload)?),
            OCPPCallAction::SendLocalList => OCPPCallResultPayload::SendLocalList(SendLocalListResponse::deserialize(payload)?),
            OCPPCallAction::SetChargingProfile => OCPPCallResultPayload::SetChargingProfile(SetChargingProfileResponse::deserialize(payload)?),
            OCPPCallAction::StartTransaction => OCPPCallResultPayload::StartTransaction(StartTransactionResponse::deserialize(payload)?),
            OCPPCallAction::StatusNotification => OCPPCallResultPayload::StatusNotification(StatusNotificationResponse::deserialize(payload)?),
            OCPPCallAction::StopTransaction => OCPPCallResultPayload::StopTransaction(StopTransactionResponse::deserialize(payload)?),
            OCPPCallAction::TriggerMessage => OCPPCallResultPayload::TriggerMessage(TriggerMessageResponse::deserialize(payload)?),
            OCPPCallAction::UnlockConnector => OCPPCallResultPayload::UnlockConnector(UnlockConnectorResponse::deserialize(payload)?),
            OCPPCallAction::UpdateFirmware => OCPPCallResultPayload::UpdateFirmware(UpdateFirmwareResponse::deserialize(payload)?),
        };

        Ok(OCPPCallResult {
            unique_id,
            payload,
        })
    }
}

/// OCPP Call Error, sent from Server to Client
#[derive(Debug, Clone)]
pub struct OCPPCallError {
    /// Unique message ID, used to match pairs of requests/responses
    pub unique_id: String,
    /// Short error code
    pub error_code: OCPPCallErrorCode,
    /// Human readable error discription
    pub error_description: String,
    /// Error details
    pub error_details: serde_json::Value,
}

impl Serialize for OCPPCallError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        (4, &self.unique_id, &self.error_code, &self.error_description, &self.error_details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OCPPCallError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let (message_type_id, unique_id, error_code, error_description, error_details): (u8, String, OCPPCallErrorCode, String, serde_json::Value) = Deserialize::deserialize(deserializer)?;

        if message_type_id != 4 {
            return Err(de::Error::invalid_value(de::Unexpected::Unsigned(message_type_id.into()), &"Message Type ID for Call Error should be '4'"));
        }

        Ok(OCPPCallError {
            unique_id,
            error_code,
            error_description,
            error_details,
        })
    }
}

impl OCPPCallError {
    fn from_call(unique_id: &str, error_code: OCPPCallErrorCode) -> OCPPCallError {
        OCPPCallError {
            unique_id: String::from(unique_id),
            error_code,
            error_description: String::new(),
            error_details: serde_json::json!({}),
        }
    }
}

/// OCPP Call Error Code
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OCPPCallErrorCode {
    NotImplemented,
    NotSupported,
    InternalError,
    ProtocolError,
    SecurityError,
    FormationViolation,
    PropertyConstraintViolation,
    OccurenceConstraintViolation,
    TypeConstraintViolation,
    GenericError,
}

/// OCPP Call Types
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Display, Clone)]
pub enum OCPPCallPayload {
    Authorize(AuthorizeRequest),
    BootNotification(BootNotificationRequest),
    ChangeAvailability(ChangeAvailabilityRequest),
    ChangeConfiguration(ChangeConfigurationRequest),
    ClearCache(ClearCacheRequest),
    ClearChargingProfile(ClearChargingProfileRequest),
    DataTransfer(DataTransferRequest),
    DiagnosticsStatusNotification(DiagnosticsStatusNotificationRequest),
    FirmwareStatusNotification(FirmwareStatusNotificationRequest),
    GetCompositeSchedule(GetCompositeScheduleRequest),
    GetConfiguration(GetConfigurationRequest),
    GetDiagnostics(GetDiagnosticsRequest),
    GetLocalListVersion(GetLocalListVersionRequest),
    Heartbeat(HeartbeatRequest),
    MeterValues(MeterValuesRequest),
    RemoteStartTransaction(RemoteStartTransactionRequest),
    RemoteStopTransaction(RemoteStopTransactionRequest),
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

impl ocpp_json_validate::JsonValidate for OCPPCallPayload {
    fn validate(&self) -> Result<(), ocpp_json_validate::JsonValidateError> {
        match self {
            OCPPCallPayload::Authorize(req) => req.validate(),
            OCPPCallPayload::BootNotification(req) => req.validate(),
            OCPPCallPayload::ChangeAvailability(req) => req.validate(),
            OCPPCallPayload::ChangeConfiguration(req) => req.validate(),
            OCPPCallPayload::ClearCache(req) => req.validate(),
            OCPPCallPayload::ClearChargingProfile(req) => req.validate(),
            OCPPCallPayload::DataTransfer(req) => req.validate(),
            OCPPCallPayload::DiagnosticsStatusNotification(req) => req.validate(),
            OCPPCallPayload::FirmwareStatusNotification(req) => req.validate(),
            OCPPCallPayload::GetCompositeSchedule(req) => req.validate(),
            OCPPCallPayload::GetConfiguration(req) => req.validate(),
            OCPPCallPayload::GetDiagnostics(req) => req.validate(),
            OCPPCallPayload::GetLocalListVersion(req) => req.validate(),
            OCPPCallPayload::Heartbeat(req) => req.validate(),
            OCPPCallPayload::MeterValues(req) => req.validate(),
            OCPPCallPayload::RemoteStartTransaction(req) => req.validate(),
            OCPPCallPayload::RemoteStopTransaction(req) => req.validate(),
            OCPPCallPayload::Reset(req) => req.validate(),
            OCPPCallPayload::SendLocalList(req) => req.validate(),
            OCPPCallPayload::SetChargingProfile(req) => req.validate(),
            OCPPCallPayload::StartTransaction(req) => req.validate(),
            OCPPCallPayload::StatusNotification(req) => req.validate(),
            OCPPCallPayload::StopTransaction(req) => req.validate(),
            OCPPCallPayload::TriggerMessage(req) => req.validate(),
            OCPPCallPayload::UnlockConnector(req) => req.validate(),
            OCPPCallPayload::UpdateFirmware(req) => req.validate(),
        }
    }
}

/// OCPP Call Result Types
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Display, Clone)]
#[serde(untagged)]
pub enum OCPPCallResultPayload {
    Authorize(AuthorizeResponse),
    BootNotification(BootNotificationResponse),
    ChangeAvailability(ChangeAvailabilityResponse),
    ChangeConfiguration(ChangeConfigurationResponse),
    ClearCache(ClearCacheResponse),
    ClearChargingProfile(ClearChargingProfileResponse),
    DataTransfer(DataTransferResponse),
    DiagnosticsStatusNotification(DiagnosticsStatusNotificationResponse),
    FirmwareStatusNotification(FirmwareStatusNotificationResponse),
    GetCompositeSchedule(GetCompositeScheduleResponse),
    GetConfiguration(GetConfigurationResponse),
    GetDiagnostics(GetDiagnosticsResponse),
    GetLocalListVersion(GetLocalListVersionResponse),
    Heartbeat(HeartbeatResponse),
    MeterValues(MeterValuesResponse),
    RemoteStartTransaction(RemoteStartTransactionResponse),
    RemoteStopTransaction(RemoteStopTransactionResponse),
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

impl ocpp_json_validate::JsonValidate for OCPPCallResultPayload {
    fn validate(&self) -> Result<(), ocpp_json_validate::JsonValidateError> {
        match self {
            OCPPCallResultPayload::Authorize(res) => res.validate(),
            OCPPCallResultPayload::BootNotification(res) => res.validate(),
            OCPPCallResultPayload::ChangeAvailability(res) => res.validate(),
            OCPPCallResultPayload::ChangeConfiguration(res) => res.validate(),
            OCPPCallResultPayload::ClearCache(res) => res.validate(),
            OCPPCallResultPayload::ClearChargingProfile(res) => res.validate(),
            OCPPCallResultPayload::DataTransfer(res) => res.validate(),
            OCPPCallResultPayload::DiagnosticsStatusNotification(res) => res.validate(),
            OCPPCallResultPayload::FirmwareStatusNotification(res) => res.validate(),
            OCPPCallResultPayload::GetCompositeSchedule(res) => res.validate(),
            OCPPCallResultPayload::GetConfiguration(res) => res.validate(),
            OCPPCallResultPayload::GetDiagnostics(res) => res.validate(),
            OCPPCallResultPayload::GetLocalListVersion(res) => res.validate(),
            OCPPCallResultPayload::Heartbeat(res) => res.validate(),
            OCPPCallResultPayload::MeterValues(res) => res.validate(),
            OCPPCallResultPayload::RemoteStartTransaction(res) => res.validate(),
            OCPPCallResultPayload::RemoteStopTransaction(res) => res.validate(),
            OCPPCallResultPayload::Reset(res) => res.validate(),
            OCPPCallResultPayload::SendLocalList(res) => res.validate(),
            OCPPCallResultPayload::SetChargingProfile(res) => res.validate(),
            OCPPCallResultPayload::StartTransaction(res) => res.validate(),
            OCPPCallResultPayload::StatusNotification(res) => res.validate(),
            OCPPCallResultPayload::StopTransaction(res) => res.validate(),
            OCPPCallResultPayload::TriggerMessage(res) => res.validate(),
            OCPPCallResultPayload::UnlockConnector(res) => res.validate(),
            OCPPCallResultPayload::UpdateFirmware(res) => res.validate(),
        }
    }
}

/// OCPP Call Types
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, Display, PartialEq, Clone)]
pub enum OCPPCallAction {
    Authorize,
    BootNotification,
    ChangeAvailability,
    ChangeConfiguration,
    ClearCache,
    ClearChargingProfile,
    DataTransfer,
    DiagnosticsStatusNotification,
    FirmwareStatusNotification,
    GetCompositeSchedule,
    GetConfiguration,
    GetDiagnostics,
    GetLocalListVersion,
    Heartbeat,
    MeterValues,
    RemoteStartTransaction,
    RemoteStopTransaction,
    Reset,
    SendLocalList,
    SetChargingProfile,
    StartTransaction,
    StatusNotification,
    StopTransaction,
    TriggerMessage,
    UnlockConnector,
    UpdateFirmware,
}

/// Predefined methods to respond to an OCPP request. Each call type has its
/// own method, and is expected to either return a valid response or an error
/// code. OCPPCallResultBuilder may be passed an OCPPCall via the [build](OCPPCallResultBuilder::build)
/// method, and will return either a corresponding OCPPCallResult or an error
/// code as returned by the handling function.
///
/// To define a response for a call type a Boxed closure may be passed in to
/// the builder. By default any undefined method will return [OCPPCallErrorCode::NotImplemented].
/// Building the [OCPPCallResult] will consume the [OCPPCall], this is used
/// to move the `unique_id` field.
///
/// The builder may be used any number of times without being consumed.
///
/// # Example
/// ```
/// # fn ocpp_message_example() -> Result<(), serde_json::Error> {
/// use ocpp::*;
///
/// // Predefined builder based on default methods
/// let mut call_result_builder = OCPPCall::result_builder()
///     // Handle heartbeat requests
///     .for_heartbeat(Box::new(|_req| {
///         Ok(
///             HeartbeatResponse {
///                 current_time: chrono::Utc::now()
///             }
///         )
///     }));
///
/// // Many messages can be handled with the same result builder
/// let msgs = vec![
///     r#""["2", "123", "Authorize", {"idTag": "test"}]""#,
///     r#""["2", "124", "Heartbeat", {}]""#,
/// ];
///
/// // Decode each message
/// for msg in msgs.iter() {
///     let message: OCPPMessage = serde_json::from_str(msg)?;
///
///     match message {
///         OCPPMessage::Call(call) => {
///             // Invoke builder to get response
///             match call_result_builder.build(call) {
///                 Ok(res) => {
///                     // A result was returned, encode and print it
///                     println!("Response: {:#?}", serde_json::to_string(&res));
///                 },
///                 Err(e) => {
///                     // Some error occured during processing
///                     println!("Error processing request: {:#?}", e);
///                 }
///             }
///         },
///         _ => {}
///     }
/// }
///
/// # return Ok(());
/// # }
/// ```
pub struct OCPPCallResultBuilder {
    authorize: Box<dyn FnMut(AuthorizeRequest) -> Result<AuthorizeResponse, OCPPCallErrorCode>>,
    boot_notification: Box<dyn FnMut(BootNotificationRequest) -> Result<BootNotificationResponse, OCPPCallErrorCode>>,
    change_availability: Box<dyn FnMut(ChangeAvailabilityRequest) -> Result<ChangeAvailabilityResponse, OCPPCallErrorCode>>,
    change_configuration: Box<dyn FnMut(ChangeConfigurationRequest) -> Result<ChangeConfigurationResponse, OCPPCallErrorCode>>,
    clear_cache: Box<dyn FnMut(ClearCacheRequest) -> Result<ClearCacheResponse, OCPPCallErrorCode>>,
    clear_charging_profile: Box<dyn FnMut(ClearChargingProfileRequest) -> Result<ClearChargingProfileResponse, OCPPCallErrorCode>>,
    data_transfer: Box<dyn FnMut(DataTransferRequest) -> Result<DataTransferResponse, OCPPCallErrorCode>>,
    diagnostics_status_notification: Box<dyn FnMut(DiagnosticsStatusNotificationRequest) -> Result<DiagnosticsStatusNotificationResponse, OCPPCallErrorCode>>,
    firmware_status_notification: Box<dyn FnMut(FirmwareStatusNotificationRequest) -> Result<FirmwareStatusNotificationResponse, OCPPCallErrorCode>>,
    get_composite_schedule: Box<dyn FnMut(GetCompositeScheduleRequest) -> Result<GetCompositeScheduleResponse, OCPPCallErrorCode>>,
    get_configuration: Box<dyn FnMut(GetConfigurationRequest) -> Result<GetConfigurationResponse, OCPPCallErrorCode>>,
    get_diagnostics: Box<dyn FnMut(GetDiagnosticsRequest) -> Result<GetDiagnosticsResponse, OCPPCallErrorCode>>,
    get_local_list_version: Box<dyn FnMut(GetLocalListVersionRequest) -> Result<GetLocalListVersionResponse, OCPPCallErrorCode>>,
    heartbeat: Box<dyn FnMut(HeartbeatRequest) -> Result<HeartbeatResponse, OCPPCallErrorCode>>,
    meter_values: Box<dyn FnMut(MeterValuesRequest) -> Result<MeterValuesResponse, OCPPCallErrorCode>>,
    remote_start_transaction: Box<dyn FnMut(RemoteStartTransactionRequest) -> Result<RemoteStartTransactionResponse, OCPPCallErrorCode>>,
    remote_stop_transaction: Box<dyn FnMut(RemoteStopTransactionRequest) -> Result<RemoteStopTransactionResponse, OCPPCallErrorCode>>,
    reset: Box<dyn FnMut(ResetRequest) -> Result<ResetResponse, OCPPCallErrorCode>>,
    send_local_list: Box<dyn FnMut(SendLocalListRequest) -> Result<SendLocalListResponse, OCPPCallErrorCode>>,
    set_charging_profile: Box<dyn FnMut(SetChargingProfileRequest) -> Result<SetChargingProfileResponse, OCPPCallErrorCode>>,
    start_transaction: Box<dyn FnMut(StartTransactionRequest) -> Result<StartTransactionResponse, OCPPCallErrorCode>>,
    status_notification: Box<dyn FnMut(StatusNotificationRequest) -> Result<StatusNotificationResponse, OCPPCallErrorCode>>,
    stop_transaction: Box<dyn FnMut(StopTransactionRequest) -> Result<StopTransactionResponse, OCPPCallErrorCode>>,
    trigger_message: Box<dyn FnMut(TriggerMessageRequest) -> Result<TriggerMessageResponse, OCPPCallErrorCode>>,
    unlock_connector: Box<dyn FnMut(UnlockConnectorRequest) -> Result<UnlockConnectorResponse, OCPPCallErrorCode>>,
    update_firmware: Box<dyn FnMut(UpdateFirmwareRequest) -> Result<UpdateFirmwareResponse, OCPPCallErrorCode>>,
}

impl Default for OCPPCallResultBuilder {
    fn default() -> OCPPCallResultBuilder {
        OCPPCallResultBuilder {
            authorize: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            boot_notification: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            change_availability: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            change_configuration: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            clear_cache: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            clear_charging_profile: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            data_transfer: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            diagnostics_status_notification: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            firmware_status_notification: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            get_composite_schedule: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            get_configuration: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            get_diagnostics: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            get_local_list_version: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            heartbeat: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            meter_values: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            remote_start_transaction: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            remote_stop_transaction: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            reset: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            send_local_list: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            set_charging_profile: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            start_transaction: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            status_notification: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            stop_transaction: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            trigger_message: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            unlock_connector: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
            update_firmware: Box::new(|_req| Err(OCPPCallErrorCode::NotImplemented)),
        }
    }
}

impl OCPPCallResultBuilder {
    /// Set handler for Authorize requests, will return an [AuthorizeResponse]
    /// upon success.
    pub fn for_authorize(mut self, f: Box<dyn FnMut(AuthorizeRequest) -> Result<AuthorizeResponse, OCPPCallErrorCode>>) -> Self { self.authorize = f; self }
    /// Set handler for BootNotification requests, will return an [BootNotificationResponse]
    /// upon success.
    pub fn for_boot_notification(mut self, f: Box<dyn FnMut(BootNotificationRequest) -> Result<BootNotificationResponse, OCPPCallErrorCode>>) -> Self { self.boot_notification = f; self }
    /// Set handler for ChangeAvailability requests, will return an [ChangeAvailabilityResponse]
    /// upon success.
    pub fn for_change_availability(mut self, f: Box<dyn FnMut(ChangeAvailabilityRequest) -> Result<ChangeAvailabilityResponse, OCPPCallErrorCode>>) -> Self { self.change_availability = f; self }
    /// Set handler for ChangeConfiguration requests, will return an [ChangeConfigurationResponse]
    /// upon success.
    pub fn for_change_configuration(mut self, f: Box<dyn FnMut(ChangeConfigurationRequest) -> Result<ChangeConfigurationResponse, OCPPCallErrorCode>>) -> Self { self.change_configuration = f; self }
    /// Set handler for ClearCache requests, will return an [ClearCacheResponse]
    /// upon success.
    pub fn for_clear_cache(mut self, f: Box<dyn FnMut(ClearCacheRequest) -> Result<ClearCacheResponse, OCPPCallErrorCode>>) -> Self { self.clear_cache = f; self }
    /// Set handler for ClearChargingProfile requests, will return an [ClearChargingProfileResponse]
    /// upon success.
    pub fn for_clear_charging_profile(mut self, f: Box<dyn FnMut(ClearChargingProfileRequest) -> Result<ClearChargingProfileResponse, OCPPCallErrorCode>>) -> Self { self.clear_charging_profile = f; self }
    /// Set handler for DataTransfer requests, will return an [DataTransferResponse]
    /// upon success.
    pub fn for_data_transfer(mut self, f: Box<dyn FnMut(DataTransferRequest) -> Result<DataTransferResponse, OCPPCallErrorCode>>) -> Self { self.data_transfer = f; self }
    /// Set handler for DiagnosticsStatusNotification requests, will return an [DiagnosticsStatusNotificationResponse]
    /// upon success.
    pub fn for_diagnostics_status_notification(mut self, f: Box<dyn FnMut(DiagnosticsStatusNotificationRequest) -> Result<DiagnosticsStatusNotificationResponse, OCPPCallErrorCode>>) -> Self { self.diagnostics_status_notification = f; self }
    /// Set handler for FirmwareStatusNotification requests, will return an [FirmwareStatusNotificationResponse]
    /// upon success.
    pub fn for_firmware_status_notification(mut self, f: Box<dyn FnMut(FirmwareStatusNotificationRequest) -> Result<FirmwareStatusNotificationResponse, OCPPCallErrorCode>>) -> Self { self.firmware_status_notification = f; self }
    /// Set handler for GetCompositeSchedule requests, will return an [GetCompositeScheduleResponse]
    /// upon success.
    pub fn for_get_composite_schedule(mut self, f: Box<dyn FnMut(GetCompositeScheduleRequest) -> Result<GetCompositeScheduleResponse, OCPPCallErrorCode>>) -> Self { self.get_composite_schedule = f; self }
    /// Set handler for GetConfiguration requests, will return an [GetConfigurationResponse]
    /// upon success.
    pub fn for_get_configuration(mut self, f: Box<dyn FnMut(GetConfigurationRequest) -> Result<GetConfigurationResponse, OCPPCallErrorCode>>) -> Self { self.get_configuration = f; self }
    /// Set handler for GetDiagnostics requests, will return an [GetDiagnosticsResponse]
    /// upon success.
    pub fn for_get_diagnostics(mut self, f: Box<dyn FnMut(GetDiagnosticsRequest) -> Result<GetDiagnosticsResponse, OCPPCallErrorCode>>) -> Self { self.get_diagnostics = f; self }
    /// Set handler for GetLocalListVersion requests, will return an [GetLocalListVersionResponse]
    /// upon success.
    pub fn for_get_local_list_version(mut self, f: Box<dyn FnMut(GetLocalListVersionRequest) -> Result<GetLocalListVersionResponse, OCPPCallErrorCode>>) -> Self { self.get_local_list_version = f; self }
    /// Set handler for Heartbeat requests, will return an [HeartbeatResponse]
    /// upon success.
    pub fn for_heartbeat(mut self, f: Box<dyn FnMut(HeartbeatRequest) -> Result<HeartbeatResponse, OCPPCallErrorCode>>) -> Self { self.heartbeat = f; self }
    /// Set handler for MeterValues requests, will return an [MeterValuesResponse]
    /// upon success.
    pub fn for_meter_values(mut self, f: Box<dyn FnMut(MeterValuesRequest) -> Result<MeterValuesResponse, OCPPCallErrorCode>>) -> Self { self.meter_values = f; self }
    /// Set handler for RemoteStartTransaction requests, will return an [RemoteStartTransactionResponse]
    /// upon success.
    pub fn for_remote_start_transaction(mut self, f: Box<dyn FnMut(RemoteStartTransactionRequest) -> Result<RemoteStartTransactionResponse, OCPPCallErrorCode>>) -> Self { self.remote_start_transaction = f; self }
    /// Set handler for RemoteStopTransaction requests, will return an [RemoteStopTransactionResponse]
    /// upon success.
    pub fn for_remote_stop_transaction(mut self, f: Box<dyn FnMut(RemoteStopTransactionRequest) -> Result<RemoteStopTransactionResponse, OCPPCallErrorCode>>) -> Self { self.remote_stop_transaction = f; self }
    /// Set handler for Reset requests, will return an [ResetResponse]
    /// upon success.
    pub fn for_reset(mut self, f: Box<dyn FnMut(ResetRequest) -> Result<ResetResponse, OCPPCallErrorCode>>) -> Self { self.reset = f; self }
    /// Set handler for SendLocalList requests, will return an [SendLocalListResponse]
    /// upon success.
    pub fn for_send_local_list(mut self, f: Box<dyn FnMut(SendLocalListRequest) -> Result<SendLocalListResponse, OCPPCallErrorCode>>) -> Self { self.send_local_list = f; self }
    /// Set handler for SetChargingProfile requests, will return an [SetChargingProfileResponse]
    /// upon success.
    pub fn for_set_charging_profile(mut self, f: Box<dyn FnMut(SetChargingProfileRequest) -> Result<SetChargingProfileResponse, OCPPCallErrorCode>>) -> Self { self.set_charging_profile = f; self }
    /// Set handler for StartTransaction requests, will return an [StartTransactionResponse]
    /// upon success.
    pub fn for_start_transaction(mut self, f: Box<dyn FnMut(StartTransactionRequest) -> Result<StartTransactionResponse, OCPPCallErrorCode>>) -> Self { self.start_transaction = f; self }
    /// Set handler for StatusNotification requests, will return an [StatusNotificationResponse]
    /// upon success.
    pub fn for_status_notification(mut self, f: Box<dyn FnMut(StatusNotificationRequest) -> Result<StatusNotificationResponse, OCPPCallErrorCode>>) -> Self { self.status_notification = f; self }
    /// Set handler for StopTransaction requests, will return an [StopTransactionResponse]
    /// upon success.
    pub fn for_stop_transaction(mut self, f: Box<dyn FnMut(StopTransactionRequest) -> Result<StopTransactionResponse, OCPPCallErrorCode>>) -> Self { self.stop_transaction = f; self }
    /// Set handler for TriggerMessage requests, will return an [TriggerMessageResponse]
    /// upon success.
    pub fn for_trigger_message(mut self, f: Box<dyn FnMut(TriggerMessageRequest) -> Result<TriggerMessageResponse, OCPPCallErrorCode>>) -> Self { self.trigger_message = f; self }
    /// Set handler for UnlockConnector requests, will return an [UnlockConnectorResponse]
    /// upon success.
    pub fn for_unlock_connector(mut self, f: Box<dyn FnMut(UnlockConnectorRequest) -> Result<UnlockConnectorResponse, OCPPCallErrorCode>>) -> Self { self.unlock_connector = f; self }
    /// Set handler for UpdateFirmware requests, will return an [UpdateFirmwareResponse]
    /// upon success.
    pub fn for_update_firmware(mut self, f: Box<dyn FnMut(UpdateFirmwareRequest) -> Result<UpdateFirmwareResponse, OCPPCallErrorCode>>) -> Self { self.update_firmware = f; self }

    /// Build [OCPPCallResult] from [OCPPCall]
    pub fn build(&mut self, call: OCPPCall) -> Result<OCPPCallResult, OCPPCallError> {
        let OCPPCall {
            unique_id,
            payload,
            ..
        } = call;

        // Validate incoming payload
        if let Err(e) = payload.validate() {
            tracing::warn!("OCPP Request Invalid: {:?}", e);
            return Err(OCPPCallError::from_call(&unique_id, OCPPCallErrorCode::ProtocolError));
        }

        let payload = match payload {
            OCPPCallPayload::Authorize(req) => (self.authorize)(req).map(|r| OCPPCallResultPayload::Authorize(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::BootNotification(req) => (self.boot_notification)(req).map(|r| OCPPCallResultPayload::BootNotification(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::ChangeAvailability(req) => (self.change_availability)(req).map(|r| OCPPCallResultPayload::ChangeAvailability(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::ChangeConfiguration(req) => (self.change_configuration)(req).map(|r| OCPPCallResultPayload::ChangeConfiguration(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::ClearCache(req) => (self.clear_cache)(req).map(|r| OCPPCallResultPayload::ClearCache(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::ClearChargingProfile(req) => (self.clear_charging_profile)(req).map(|r| OCPPCallResultPayload::ClearChargingProfile(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::DataTransfer(req) => (self.data_transfer)(req).map(|r| OCPPCallResultPayload::DataTransfer(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::DiagnosticsStatusNotification(req) => (self.diagnostics_status_notification)(req).map(|r| OCPPCallResultPayload::DiagnosticsStatusNotification(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::FirmwareStatusNotification(req) => (self.firmware_status_notification)(req).map(|r| OCPPCallResultPayload::FirmwareStatusNotification(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::GetCompositeSchedule(req) => (self.get_composite_schedule)(req).map(|r| OCPPCallResultPayload::GetCompositeSchedule(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::GetConfiguration(req) => (self.get_configuration)(req).map(|r| OCPPCallResultPayload::GetConfiguration(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::GetDiagnostics(req) => (self.get_diagnostics)(req).map(|r| OCPPCallResultPayload::GetDiagnostics(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::GetLocalListVersion(req) => (self.get_local_list_version)(req).map(|r| OCPPCallResultPayload::GetLocalListVersion(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::Heartbeat(req) => (self.heartbeat)(req).map(|r| OCPPCallResultPayload::Heartbeat(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::MeterValues(req) => (self.meter_values)(req).map(|r| OCPPCallResultPayload::MeterValues(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::RemoteStartTransaction(req) => (self.remote_start_transaction)(req).map(|r| OCPPCallResultPayload::RemoteStartTransaction(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::RemoteStopTransaction(req) => (self.remote_stop_transaction)(req).map(|r| OCPPCallResultPayload::RemoteStopTransaction(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::Reset(req) => (self.reset)(req).map(|r| OCPPCallResultPayload::Reset(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::SendLocalList(req) => (self.send_local_list)(req).map(|r| OCPPCallResultPayload::SendLocalList(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::SetChargingProfile(req) => (self.set_charging_profile)(req).map(|r| OCPPCallResultPayload::SetChargingProfile(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::StartTransaction(req) => (self.start_transaction)(req).map(|r| OCPPCallResultPayload::StartTransaction(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::StatusNotification(req) => (self.status_notification)(req).map(|r| OCPPCallResultPayload::StatusNotification(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::StopTransaction(req) => (self.stop_transaction)(req).map(|r| OCPPCallResultPayload::StopTransaction(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::TriggerMessage(req) => (self.trigger_message)(req).map(|r| OCPPCallResultPayload::TriggerMessage(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::UnlockConnector(req) => (self.unlock_connector)(req).map(|r| OCPPCallResultPayload::UnlockConnector(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::UpdateFirmware(req) => (self.update_firmware)(req).map(|r| OCPPCallResultPayload::UpdateFirmware(r)).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
        };

        // Validate outgoing payload
        if let Err(e) = payload.validate() {
            tracing::error!("OCPP Response Invalid: {:?}", e);
            return Err(OCPPCallError::from_call(&unique_id, OCPPCallErrorCode::InternalError));
        }

        return Ok(OCPPCallResult {
            unique_id,
            payload,
        });
    }
}
