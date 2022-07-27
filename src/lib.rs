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
//! struct MyResultBuilder {}
//! impl OCPPCallResultBuilder for MyResultBuilder {
//!     fn authorize(&mut self, req: AuthorizeRequest) -> Result<AuthorizeResponse, OCPPCallErrorCode> {
//!         let status = if req.id_tag == "valid_tag" {
//!             AuthorizationStatus::Accepted
//!         } else {
//!             AuthorizationStatus::Blocked
//!         };
//!
//!         Ok(AuthorizeResponse {
//!             id_tag_info: IdTagInfo {
//!                 expiry_date: None,
//!                 parent_id_tag: None,
//!                 status,
//!             }
//!         })
//!     }
//!
//!     fn heartbeat(&mut self, _req: HeartbeatRequest) -> Result<HeartbeatResponse, OCPPCallErrorCode> {
//!         Ok(
//!             HeartbeatResponse {
//!                 current_time: chrono::Utc::now()
//!             }
//!         )
//!     }
//!
//!     fn boot_notification(&mut self, _req: BootNotificationRequest) -> Result<BootNotificationResponse, OCPPCallErrorCode> {
//!         Ok(
//!             BootNotificationResponse {
//!                 status: BootNotificationStatus::Accepted,
//!                 current_time: chrono::Utc::now(),
//!                 interval: 5,
//!             }
//!         )
//!     }
//! }
//!
//! let mut call_result_builder = MyResultBuilder {};
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
//!         match call_result_builder.build_response(call) {
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
pub mod error;
pub mod macros;
pub mod point_init;
pub mod server_init;

#[cfg(test)]
pub mod test;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::Display;

use ocpp_json_validate::JsonValidate;
use thiserror::Error;

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
    where
        S: Serializer,
    {
        (2, &self.unique_id, &self.action, &self.payload).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OCPPCall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
                return Err(de::Error::unknown_variant(
                    &action,
                    &[
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
                    ],
                ));
            }
        };

        Ok(OCPPCall { unique_id, action, payload })
    }
}

impl From<(String, OCPPCallPayload)> for OCPPCall {
    fn from(from: (String, OCPPCallPayload)) -> OCPPCall {
        let (unique_id, payload) = from;
        let action = String::from(match payload {
            OCPPCallPayload::Authorize(_) => "Authorize",
            OCPPCallPayload::BootNotification(_) => "BootNotification",
            OCPPCallPayload::ChangeAvailability(_) => "ChangeAvailability",
            OCPPCallPayload::ChangeConfiguration(_) => "ChangeConfiguration",
            OCPPCallPayload::ClearCache(_) => "ClearCache",
            OCPPCallPayload::ClearChargingProfile(_) => "ClearChargingProfile",
            OCPPCallPayload::DataTransfer(_) => "DataTransfer",
            OCPPCallPayload::DiagnosticsStatusNotification(_) => "DiagnosticsStatusNotification",
            OCPPCallPayload::FirmwareStatusNotification(_) => "FirmwareStatusNotification",
            OCPPCallPayload::GetCompositeSchedule(_) => "GetCompositeSchedule",
            OCPPCallPayload::GetConfiguration(_) => "GetConfiguration",
            OCPPCallPayload::GetDiagnostics(_) => "GetDiagnostics",
            OCPPCallPayload::GetLocalListVersion(_) => "GetLocalListVersion",
            OCPPCallPayload::Heartbeat(_) => "Heartbeat",
            OCPPCallPayload::MeterValues(_) => "MeterValues",
            OCPPCallPayload::RemoteStartTransaction(_) => "RemoteStartTransaction",
            OCPPCallPayload::RemoteStopTransaction(_) => "RemoteStopTransaction",
            OCPPCallPayload::Reset(_) => "Reset",
            OCPPCallPayload::SendLocalList(_) => "SendLocalList",
            OCPPCallPayload::SetChargingProfile(_) => "SetChargingProfile",
            OCPPCallPayload::StartTransaction(_) => "StartTransaction",
            OCPPCallPayload::StatusNotification(_) => "StatusNotification",
            OCPPCallPayload::StopTransaction(_) => "StopTransaction",
            OCPPCallPayload::TriggerMessage(_) => "TriggerMessage",
            OCPPCallPayload::UnlockConnector(_) => "UnlockConnector",
            OCPPCallPayload::UpdateFirmware(_) => "UpdateFirmware",
        });

        OCPPCall { unique_id, action, payload }
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
    where
        S: Serializer,
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
    where
        D: Deserializer<'de>,
    {
        let (message_type_id, unique_id, payload): (u8, String, serde_json::Value) = Deserialize::deserialize(deserializer)?;

        if message_type_id != 3 {
            return Err(de::Error::invalid_value(de::Unexpected::Unsigned(message_type_id.into()), &"Message Type ID for Call Result should be '3'"));
        }

        Ok(OCPPCallResultUnknown { unique_id, payload })
    }
}

impl OCPPCallResult {
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
    pub fn from_unknown(action: &OCPPCallAction, unknown: OCPPCallResultUnknown) -> Result<Self, serde_json::Error> {
        let OCPPCallResultUnknown { unique_id, payload } = unknown;

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

        Ok(OCPPCallResult { unique_id, payload })
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
    where
        S: Serializer,
    {
        (4, &self.unique_id, &self.error_code, &self.error_description, &self.error_details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OCPPCallError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
#[derive(Serialize, Deserialize, Debug, Clone, Error)]
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

impl std::fmt::Display for OCPPCallErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:?}", self) }
}

/// OCPP Call Types
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Display, Clone)]
#[serde(untagged)]
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

impl OCPPCallPayload {
    /// Get an OCPPResult from the type of its original call.
    pub fn get_result_from_call(&self, result: OCPPCallResultUnknown) -> Result<OCPPCallResultPayload, serde_json::error::Error> {
        use OCPPCallPayload::*;

        let output = match self {
            Authorize(_) => OCPPCallResult::from_unknown(&OCPPCallAction::Authorize, result)?,
            BootNotification(_) => OCPPCallResult::from_unknown(&OCPPCallAction::BootNotification, result)?,
            ChangeAvailability(_) => OCPPCallResult::from_unknown(&OCPPCallAction::ChangeAvailability, result)?,
            ChangeConfiguration(_) => OCPPCallResult::from_unknown(&OCPPCallAction::ChangeConfiguration, result)?,
            ClearCache(_) => OCPPCallResult::from_unknown(&OCPPCallAction::ClearCache, result)?,
            ClearChargingProfile(_) => OCPPCallResult::from_unknown(&OCPPCallAction::ClearChargingProfile, result)?,
            DataTransfer(_) => OCPPCallResult::from_unknown(&OCPPCallAction::DataTransfer, result)?,
            DiagnosticsStatusNotification(_) => OCPPCallResult::from_unknown(&OCPPCallAction::DiagnosticsStatusNotification, result)?,
            FirmwareStatusNotification(_) => OCPPCallResult::from_unknown(&OCPPCallAction::FirmwareStatusNotification, result)?,
            GetCompositeSchedule(_) => OCPPCallResult::from_unknown(&OCPPCallAction::GetCompositeSchedule, result)?,
            GetConfiguration(_) => OCPPCallResult::from_unknown(&OCPPCallAction::GetConfiguration, result)?,
            GetDiagnostics(_) => OCPPCallResult::from_unknown(&OCPPCallAction::GetDiagnostics, result)?,
            GetLocalListVersion(_) => OCPPCallResult::from_unknown(&OCPPCallAction::GetLocalListVersion, result)?,
            Heartbeat(_) => OCPPCallResult::from_unknown(&OCPPCallAction::Heartbeat, result)?,
            MeterValues(_) => OCPPCallResult::from_unknown(&OCPPCallAction::MeterValues, result)?,
            RemoteStartTransaction(_) => OCPPCallResult::from_unknown(&OCPPCallAction::RemoteStartTransaction, result)?,
            RemoteStopTransaction(_) => OCPPCallResult::from_unknown(&OCPPCallAction::RemoteStopTransaction, result)?,
            Reset(_) => OCPPCallResult::from_unknown(&OCPPCallAction::Reset, result)?,
            SendLocalList(_) => OCPPCallResult::from_unknown(&OCPPCallAction::SendLocalList, result)?,
            SetChargingProfile(_) => OCPPCallResult::from_unknown(&OCPPCallAction::SetChargingProfile, result)?,
            StartTransaction(_) => OCPPCallResult::from_unknown(&OCPPCallAction::StartTransaction, result)?,
            StatusNotification(_) => OCPPCallResult::from_unknown(&OCPPCallAction::StatusNotification, result)?,
            StopTransaction(_) => OCPPCallResult::from_unknown(&OCPPCallAction::StopTransaction, result)?,
            TriggerMessage(_) => OCPPCallResult::from_unknown(&OCPPCallAction::TriggerMessage, result)?,
            UnlockConnector(_) => OCPPCallResult::from_unknown(&OCPPCallAction::UnlockConnector, result)?,
            UpdateFirmware(_) => OCPPCallResult::from_unknown(&OCPPCallAction::UpdateFirmware, result)?,
        };

        return Ok(output.payload);
    }
}

impl ocpp_json_validate::JsonValidate for OCPPCallPayload {
    fn schema_validate(&self) -> Result<(), ocpp_json_validate::JsonValidateError> {
        match self {
            OCPPCallPayload::Authorize(req) => req.schema_validate(),
            OCPPCallPayload::BootNotification(req) => req.schema_validate(),
            OCPPCallPayload::ChangeAvailability(req) => req.schema_validate(),
            OCPPCallPayload::ChangeConfiguration(req) => req.schema_validate(),
            OCPPCallPayload::ClearCache(req) => req.schema_validate(),
            OCPPCallPayload::ClearChargingProfile(req) => req.schema_validate(),
            OCPPCallPayload::DataTransfer(req) => req.schema_validate(),
            OCPPCallPayload::DiagnosticsStatusNotification(req) => req.schema_validate(),
            OCPPCallPayload::FirmwareStatusNotification(req) => req.schema_validate(),
            OCPPCallPayload::GetCompositeSchedule(req) => req.schema_validate(),
            OCPPCallPayload::GetConfiguration(req) => req.schema_validate(),
            OCPPCallPayload::GetDiagnostics(req) => req.schema_validate(),
            OCPPCallPayload::GetLocalListVersion(req) => req.schema_validate(),
            OCPPCallPayload::Heartbeat(req) => req.schema_validate(),
            OCPPCallPayload::MeterValues(req) => req.schema_validate(),
            OCPPCallPayload::RemoteStartTransaction(req) => req.schema_validate(),
            OCPPCallPayload::RemoteStopTransaction(req) => req.schema_validate(),
            OCPPCallPayload::Reset(req) => req.schema_validate(),
            OCPPCallPayload::SendLocalList(req) => req.schema_validate(),
            OCPPCallPayload::SetChargingProfile(req) => req.schema_validate(),
            OCPPCallPayload::StartTransaction(req) => req.schema_validate(),
            OCPPCallPayload::StatusNotification(req) => req.schema_validate(),
            OCPPCallPayload::StopTransaction(req) => req.schema_validate(),
            OCPPCallPayload::TriggerMessage(req) => req.schema_validate(),
            OCPPCallPayload::UnlockConnector(req) => req.schema_validate(),
            OCPPCallPayload::UpdateFirmware(req) => req.schema_validate(),
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
    fn schema_validate(&self) -> Result<(), ocpp_json_validate::JsonValidateError> {
        match self {
            OCPPCallResultPayload::Authorize(r) => r.schema_validate(),
            OCPPCallResultPayload::BootNotification(r) => r.schema_validate(),
            OCPPCallResultPayload::ChangeAvailability(r) => r.schema_validate(),
            OCPPCallResultPayload::ChangeConfiguration(r) => r.schema_validate(),
            OCPPCallResultPayload::ClearCache(r) => r.schema_validate(),
            OCPPCallResultPayload::ClearChargingProfile(r) => r.schema_validate(),
            OCPPCallResultPayload::DataTransfer(r) => r.schema_validate(),
            OCPPCallResultPayload::DiagnosticsStatusNotification(r) => r.schema_validate(),
            OCPPCallResultPayload::FirmwareStatusNotification(r) => r.schema_validate(),
            OCPPCallResultPayload::GetCompositeSchedule(r) => r.schema_validate(),
            OCPPCallResultPayload::GetConfiguration(r) => r.schema_validate(),
            OCPPCallResultPayload::GetDiagnostics(r) => r.schema_validate(),
            OCPPCallResultPayload::GetLocalListVersion(r) => r.schema_validate(),
            OCPPCallResultPayload::Heartbeat(r) => r.schema_validate(),
            OCPPCallResultPayload::MeterValues(r) => r.schema_validate(),
            OCPPCallResultPayload::RemoteStartTransaction(r) => r.schema_validate(),
            OCPPCallResultPayload::RemoteStopTransaction(r) => r.schema_validate(),
            OCPPCallResultPayload::Reset(r) => r.schema_validate(),
            OCPPCallResultPayload::SendLocalList(r) => r.schema_validate(),
            OCPPCallResultPayload::SetChargingProfile(r) => r.schema_validate(),
            OCPPCallResultPayload::StartTransaction(r) => r.schema_validate(),
            OCPPCallResultPayload::StatusNotification(r) => r.schema_validate(),
            OCPPCallResultPayload::StopTransaction(r) => r.schema_validate(),
            OCPPCallResultPayload::TriggerMessage(r) => r.schema_validate(),
            OCPPCallResultPayload::UnlockConnector(r) => r.schema_validate(),
            OCPPCallResultPayload::UpdateFirmware(r) => r.schema_validate(),
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
/// // Implement OCPPCallResultBuilder
/// struct MyResultBuilder {}
/// impl OCPPCallResultBuilder for MyResultBuilder {
///     fn heartbeat(&mut self, _req: HeartbeatRequest) -> Result<HeartbeatResponse, OCPPCallErrorCode> {
///         Ok(
///             HeartbeatResponse {
///                 current_time: chrono::Utc::now()
///             }
///         )
///     }
/// }
///
/// let mut call_result_builder = MyResultBuilder {};
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
///             match call_result_builder.build_response(call) {
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
pub trait OCPPCallResultBuilder {
    /// Handle AuthorizeRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn authorize(&mut self, _req: AuthorizeRequest) -> Result<AuthorizeResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle BootNotificationRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn boot_notification(&mut self, _req: BootNotificationRequest) -> Result<BootNotificationResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle ChangeAvailabilityRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn change_availability(&mut self, _req: ChangeAvailabilityRequest) -> Result<ChangeAvailabilityResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle ChangeConfigurationRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn change_configuration(&mut self, _req: ChangeConfigurationRequest) -> Result<ChangeConfigurationResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle ClearCacheRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn clear_cache(&mut self, _req: ClearCacheRequest) -> Result<ClearCacheResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle ClearChargingProfileRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn clear_charging_profile(&mut self, _req: ClearChargingProfileRequest) -> Result<ClearChargingProfileResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle DataTransferRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn data_transfer(&mut self, _req: DataTransferRequest) -> Result<DataTransferResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle DiagnosticsStatusNotificationRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn diagnostics_status_notification(&mut self, _req: DiagnosticsStatusNotificationRequest) -> Result<DiagnosticsStatusNotificationResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle FirmwareStatusNotificationRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn firmware_status_notification(&mut self, _req: FirmwareStatusNotificationRequest) -> Result<FirmwareStatusNotificationResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle GetCompositeScheduleRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn get_composite_schedule(&mut self, _req: GetCompositeScheduleRequest) -> Result<GetCompositeScheduleResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle GetConfigurationRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn get_configuration(&mut self, _req: GetConfigurationRequest) -> Result<GetConfigurationResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle GetDiagnosticsRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn get_diagnostics(&mut self, _req: GetDiagnosticsRequest) -> Result<GetDiagnosticsResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle GetLocalListVersionRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn get_local_list_version(&mut self, _req: GetLocalListVersionRequest) -> Result<GetLocalListVersionResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle HeartbeatRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn heartbeat(&mut self, _req: HeartbeatRequest) -> Result<HeartbeatResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle MeterValuesRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn meter_values(&mut self, _req: MeterValuesRequest) -> Result<MeterValuesResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle RemoteStartTransactionRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn remote_start_transaction(&mut self, _req: RemoteStartTransactionRequest) -> Result<RemoteStartTransactionResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle RemoteStopTransactionRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn remote_stop_transaction(&mut self, _req: RemoteStopTransactionRequest) -> Result<RemoteStopTransactionResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle ResetRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn reset(&mut self, _req: ResetRequest) -> Result<ResetResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle SendLocalListRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn send_local_list(&mut self, _req: SendLocalListRequest) -> Result<SendLocalListResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle SetChargingProfileRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn set_charging_profile(&mut self, _req: SetChargingProfileRequest) -> Result<SetChargingProfileResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle StartTransactionRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn start_transaction(&mut self, _req: StartTransactionRequest) -> Result<StartTransactionResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle StatusNotificationRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn status_notification(&mut self, _req: StatusNotificationRequest) -> Result<StatusNotificationResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle StopTransactionRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn stop_transaction(&mut self, _req: StopTransactionRequest) -> Result<StopTransactionResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle TriggerMessageRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn trigger_message(&mut self, _req: TriggerMessageRequest) -> Result<TriggerMessageResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle UnlockConnectorRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn unlock_connector(&mut self, _req: UnlockConnectorRequest) -> Result<UnlockConnectorResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }
    /// Handle UpdateFirmwareRequest. Returns [OCPPCallErrorCode::NotImplemented] by default.
    fn update_firmware(&mut self, _req: UpdateFirmwareRequest) -> Result<UpdateFirmwareResponse, OCPPCallErrorCode> { Err(OCPPCallErrorCode::NotImplemented) }

    /// Build [OCPPCallResult] from [OCPPCall]
    fn build_response(&mut self, call: OCPPCall) -> Result<OCPPCallResult, OCPPCallError> {
        let OCPPCall { unique_id, payload, .. } = call;

        // Validate incoming payload
        if let Err(e) = payload.schema_validate() {
            tracing::warn!("OCPP Request Invalid: {:?}", e);
            return Err(OCPPCallError::from_call(&unique_id, OCPPCallErrorCode::ProtocolError));
        }

        let payload = match payload {
            OCPPCallPayload::Authorize(req) => self.authorize(req).map(OCPPCallResultPayload::Authorize).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::BootNotification(req) => self.boot_notification(req).map(OCPPCallResultPayload::BootNotification).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::ChangeAvailability(req) => self.change_availability(req).map(OCPPCallResultPayload::ChangeAvailability).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::ChangeConfiguration(req) => self.change_configuration(req).map(OCPPCallResultPayload::ChangeConfiguration).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::ClearCache(req) => self.clear_cache(req).map(OCPPCallResultPayload::ClearCache).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::ClearChargingProfile(req) => self.clear_charging_profile(req).map(OCPPCallResultPayload::ClearChargingProfile).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::DataTransfer(req) => self.data_transfer(req).map(OCPPCallResultPayload::DataTransfer).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::DiagnosticsStatusNotification(req) => self.diagnostics_status_notification(req).map(OCPPCallResultPayload::DiagnosticsStatusNotification).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::FirmwareStatusNotification(req) => self.firmware_status_notification(req).map(OCPPCallResultPayload::FirmwareStatusNotification).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::GetCompositeSchedule(req) => self.get_composite_schedule(req).map(OCPPCallResultPayload::GetCompositeSchedule).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::GetConfiguration(req) => self.get_configuration(req).map(OCPPCallResultPayload::GetConfiguration).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::GetDiagnostics(req) => self.get_diagnostics(req).map(OCPPCallResultPayload::GetDiagnostics).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::GetLocalListVersion(req) => self.get_local_list_version(req).map(OCPPCallResultPayload::GetLocalListVersion).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::Heartbeat(req) => self.heartbeat(req).map(OCPPCallResultPayload::Heartbeat).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::MeterValues(req) => self.meter_values(req).map(OCPPCallResultPayload::MeterValues).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::RemoteStartTransaction(req) => self.remote_start_transaction(req).map(OCPPCallResultPayload::RemoteStartTransaction).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::RemoteStopTransaction(req) => self.remote_stop_transaction(req).map(OCPPCallResultPayload::RemoteStopTransaction).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::Reset(req) => self.reset(req).map(OCPPCallResultPayload::Reset).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::SendLocalList(req) => self.send_local_list(req).map(OCPPCallResultPayload::SendLocalList).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::SetChargingProfile(req) => self.set_charging_profile(req).map(OCPPCallResultPayload::SetChargingProfile).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::StartTransaction(req) => self.start_transaction(req).map(OCPPCallResultPayload::StartTransaction).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::StatusNotification(req) => self.status_notification(req).map(OCPPCallResultPayload::StatusNotification).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::StopTransaction(req) => self.stop_transaction(req).map(OCPPCallResultPayload::StopTransaction).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::TriggerMessage(req) => self.trigger_message(req).map(OCPPCallResultPayload::TriggerMessage).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::UnlockConnector(req) => self.unlock_connector(req).map(OCPPCallResultPayload::UnlockConnector).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
            OCPPCallPayload::UpdateFirmware(req) => self.update_firmware(req).map(OCPPCallResultPayload::UpdateFirmware).map_err(|e| OCPPCallError::from_call(&unique_id, e))?,
        };

        // Validate outgoing payload
        if let Err(e) = payload.schema_validate() {
            tracing::error!("OCPP Response Invalid: {:?}", e);
            return Err(OCPPCallError::from_call(&unique_id, OCPPCallErrorCode::InternalError));
        }

        Ok(OCPPCallResult { unique_id, payload })
    }
}
