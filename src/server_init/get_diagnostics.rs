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

use chrono::{DateTime, Utc};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/GetDiagnostics.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsRequest {
    pub location: String,
    pub retries: Option<u32>,
    pub retry_interval: Option<u32>,
    pub start_time: Option<DateTime<Utc>>,
    pub stop_time: Option<DateTime<Utc>>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/GetDiagnosticsResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsResponse {
    pub file_name: String,
}
