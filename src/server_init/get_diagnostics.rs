//! Server request for a ChargePoint to send diagnostic information
//!
//! # Behaviour
//! The Central System SHALL send a GetDiagnostics.req PDU for getting diagnostic information of a Charge Point with a
//! location where the Charge Point MUST upload its diagnostic data to and optionally a begin and end time for the requested diagnostic information.
//!
//! # Response
//! Upon receipt of a GetDiagnostics.req PDU, and if diagnostics information is available then Charge Point SHALL respond
//! with a GetDiagnostics.conf PDU stating the name of the file containing the diagnostic information that will be uploaded.
//! Charge Point SHALL upload a single file. Format of the diagnostics file is not prescribed. If no diagnostics file is available,
//! then GetDiagnostics.conf SHALL NOT contain a file name.
//!
//! During uploading of a diagnostics file, the Charge Point MUST send DiagnosticsStatusNotification.req PDUs to keep the Central
//! System updated with the status of the upload process.

use crate::ocpp_json_validate::{self, json_validate};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/GetDiagnostics.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the GetDiagnostics.req PDU sent by the Central System to the Charge Point.
pub struct GetDiagnosticsRequest {
    /// Required. This contains the location (directory) where the diagnostics file shall be uploaded to.
    pub location: String,
    /// Optional. This specifies how many times Charge Point must try to upload the diagnostics before giving up. If this field is not present, it is left to Charge Point to decide how many times it wants to retry.
    pub retries: Option<u32>,
    /// Optional. The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charge Point to decide how long to wait between attempts.
    pub retry_interval: Option<u32>,
    /// Optional. This contains the date and time of the oldest logging information to include in the diagnostics.
    pub start_time: Option<DateTime<Utc>>,
    /// Optional. This contains the date and time of the latest logging information to include in the diagnostics.
    pub stop_time: Option<DateTime<Utc>>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/GetDiagnosticsResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the GetDiagnostics.conf PDU sent by the Charge Point to the Central System in response to a GetDiagnostics.req PDU.
pub struct GetDiagnosticsResponse {
    /// Optional. This contains the name of the file with diagnostic information that will be uploaded. This field is not present when no diagnostic information is available.
    pub file_name: String,
}
