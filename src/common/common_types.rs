//! A collection of shared types used by mutiple message structures
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

///Generic status message denoting Accepted or Rejected state.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
pub enum SimpleStatus {
    /// Command will be executed.
    Accepted,
    /// Command will not be executed.
    Rejected,
}
/// Contains status information about an identifier. It is returned in [Authorize.req](crate::point_init::authorize), [StartTransaction.conf](crate::point_init::start_transaction) and [StopTransaction.conf](crate::point_init::stop_transaction).
///
/// If expiryDate is not given, the status has no end date.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    /// Optional. This contains the date at which idTag should be removed from the Authorization Cache.
    pub expiry_date: Option<DateTime<Utc>>,
    /// Optional. This contains the parent-identifier. IdToken
    pub parent_id_tag: Option<String>,
    /// Required. This contains whether the idTag has been accepted or not by the Central System.
    pub status: AuthorizationStatus,
}

/// Status in a response to an AuthorizeRequest
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
pub enum AuthorizationStatus {
    /// Identifier is allowed for charging.
    Accepted,
    /// Identifier has been blocked. Not allowed for charging.
    Blocked,
    /// Identifier has expired. Not allowed for charging.
    Expired,
    /// Identifier is unknown. Not allowed for charging.
    Invalid,
    /// Identifier is already involved in another transaction and multiple transactions are not allowed. (Only relevant for a StartTransaction.req.)
    ConcurrentTx,
}
