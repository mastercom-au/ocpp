//! # ocpp
//!
//! This library is intended to provide a framework for serialising and deserialising OCPP packets as rust types.
//! The original OCPP protocol is an [Open Charge Alliance ](https://www.openchargealliance.org/) project
//!
#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;

pub mod common;
pub mod point_init;
pub mod server_init;

#[cfg(test)]
pub mod test;



use strum_macros::Display;
use serde::{de, Deserialize, Serialize, Deserializer, Serializer};

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
///         let response = match call.payload {
///             // A heartbeat request
///             OCPPCallPayload::Heartbeat(_req) => {
///                 // Build response based on request
///                 OCPPMessage::CallResult(OCPPCallResult {
///                     unique_id: call.unique_id,
///                     payload: OCPPCallResultPayload::Heartbeat(HeartbeatResponse {
///                         current_time: Utc::now()
///                     })
///                 })
///             },
/// #             _ => return Ok(())
///         };
///
///         // Encode response and print it
///         println!("Response: {:#?}", serde_json::to_string(&response));
///     },
///     _ => {}
/// }
/// # return Ok(());
/// # }
/// ```
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
    ///     let result = OCPPCallResult::from_unknown(OCPPCallAction::StatusNotification, unknown)?;
    ///     println!("Decoded status notification response: {:#?}", result)
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_unknown(action: OCPPCallAction, unknown: OCPPCallResultUnknown) -> Result<Self, serde_json::Error>
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
    pub error_code: String,
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
        let (message_type_id, unique_id, error_code, error_description, error_details): (u8, String, String, String, serde_json::Value) = Deserialize::deserialize(deserializer)?;

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
