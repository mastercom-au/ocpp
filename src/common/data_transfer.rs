//! Message type for handling large data packets.
//!
//! If a Charge Point needs to send information to the Central System for a function not supported by OCPP, it
//! SHALL use the DataTransfer.req PDU.
//! The vendorId in the request SHOULD be known to the Central System and uniquely identify the vendor-specific
//! implementation. The VendorId SHOULD be a value from the reversed DNS namespace, where the top tiers of the
//! name, when reversed, should correspond to the publicly registered primary DNS name of the Vendor
//! organisation.
//!
//! Optionally, the messageId in the request PDU MAY be used to indicate a specific message or implementation.
//! The length of data in both the request and response PDU is undefined and should be agreed upon by all parties
//! involved.
//!
//! If the recipient of the request has no implementation for the specific vendorId it SHALL return a status
//! ‘UnknownVendor’ and the data element SHALL not be present. In case of a messageId mismatch (if used) the
//! recipient SHALL return status ‘UnknownMessageId’. In all other cases the usage of status ‘Accepted’ or ‘Rejected’
//! and the data element is part of the vendor-specific agreement between the parties involved.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/DataTransfer.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    /// Required. This identifies the Vendor specific implementation
    pub vendor_id: String,
    /// Optional. Additional identification field
    pub message_id: Option<String>,
    /// Optional. Data without specified length or format.
    pub data: Option<String>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/DataTransferResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    /// Required. This indicates the success or failure of the data transfer.
    pub status: DataTransferStatus,
    /// Optional. Data in response to request.
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum DataTransferStatus {
    /// Message has been accepted and the contained request is accepted.
    Accepted,
    /// Message has been accepted but the contained request is rejected.
    Rejected,
    /// Message could not be interpreted due to unknown messageId string.
    UnknownMessageId,
    /// Message could not be interpreted due to unknown vendorId string.
    UnknownVendorId,
}
