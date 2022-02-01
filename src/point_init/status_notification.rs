use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/StatusNotification.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    pub connector_id: u32,
    pub error_code: StatusNotificationErrorCode,
    pub info: Option<String>,
    pub status: StatusNotificationStatus,
    pub timestamp: Option<DateTime<Utc>>,
    pub vendor_id: Option<String>,
    pub vendor_error_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StatusNotificationErrorCode {
    ConnectorLockFailure,
    EVCommunicationError,
    GroundFailure,
    HighTemperature,
    InternalError,
    LocalListConflict,
    NoError,
    OtherError,
    OverCurrentFailure,
    PowerMeterFailure,
    PowerSwitchFailure,
    ReaderFailure,
    ResetFailure,
    UnderVoltage,
    OverVoltage,
    WeakSignal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StatusNotificationStatus {
    Available,
    Preparing,
    Charging,
    SuspendedEVSE,
    SuspendedEV,
    Finishing,
    Reserved,
    Unavailable,
    Faulted,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/StatusNotificationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationResponse {}
