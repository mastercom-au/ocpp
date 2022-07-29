//! A collection of shared types used by mutiple message structures
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
/// Newtype over Time field to allow property testing and validation
pub struct UtcTime(DateTime<Utc>);

/// Lets us operate on this newtype as if it were the inner type
impl std::ops::Deref for UtcTime {
    type Target = DateTime<Utc>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Allows .into() syntax for DateTime<Utc>
impl std::convert::From<DateTime<Utc>> for UtcTime {
    fn from(t: DateTime<Utc>) -> Self { Self(t) }
}

/// Arbitrary trait allows this value to be fuzzed by proptest
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for UtcTime {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use chrono::TimeZone;
        use proptest::arbitrary::any;
        use proptest::strategy::Strategy;

        any::<i64>().prop_map(|z| UtcTime(Utc.timestamp_nanos(z))).boxed()
    }

    fn arbitrary() -> Self::Strategy { Self::arbitrary_with(Default::default()) }
}

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

#[cfg(test)]
mod testing {
    use super::*;
    use chrono::TimeZone;
    use proptest::arbitrary::any;
    use proptest::strategy::{BoxedStrategy, Strategy};
    /// Arbitrary trait allows this value to be fuzzed by proptest
    impl proptest::arbitrary::Arbitrary for UtcTime {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy { any::<i64>().prop_map(|z| UtcTime(Utc.timestamp_nanos(z))).boxed() }

        fn arbitrary() -> Self::Strategy { Self::arbitrary_with(Default::default()) }
    }
}
