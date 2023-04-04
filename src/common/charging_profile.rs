//! Definition and builder for the Charge Profile structure, used to set charging behaviour and scheduling
//!
//! ```text
//! ChargePointProfile
//!     ChargingProfileId:      u32
//!     StackLevel:             Option<u32>
//!     ChargingProfilePurpose  Enum
//!     ChargingProfileKind     Enum
//!     RecurrencyKind          Option<Enum>
//!     ValidFrom               Option<DateTime<Utc>>
//!     ValidTo                 Option<DateTime<Utc>>
//!     ChargingSchedule        Obj
//!         Duration                Option<u32>
//!         StartSchedule           Option<DateTime<Utc>>
//!         ChargingRateUnit        Enum
//!         MinChargingRate         Option<f32>
//!         ChargingSchedulePeriod  Vec<Obj>
//!             StartPeriod             u32    
//!             Limit                   f32
//!             NumberPhases            Option<u32>
//! ```
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

/// A ChargingProfile consists of a ChargingSchedule, describing the amount of power or current that can be delivered per time interval.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfile {
    /// Required. Unique identifier for this profile.
    pub charging_profile_id: u32,
    /// Optional. Only valid if ChargingProfilePurpose is set to TxProfile, the transactionId MAY be used to match the profile to a specific transaction.
    pub transaction_id: Option<u32>,
    /// Required. Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values. Lowest level is 0.
    pub stack_level: u32,
    /// Required. Defines the purpose of the schedule transferred by this message.
    pub charging_profile_purpose: ChargingProfilePurpose,
    /// Required. Indicates the kind of schedule.
    pub charging_profile_kind: ChargingProfileKind,
    /// Optional. Indicates the start point of a recurrence
    pub recurrency_kind: Option<RecurrencyKind>,
    /// Optional. Point in time at which the profile starts to be valid. If absent, the profile is valid as soon as it is received by the Charge Point.
    pub valid_from: Option<DateTime<Utc>>,
    /// Optional. Point in time at which the profile stops to be valid. If absent, the profile is valid until it is replaced by another profile.
    pub valid_to: Option<DateTime<Utc>>,
    /// Required. Contains limits for the available power or current over time
    pub charging_schedule: ChargingSchedule,
}

/// Charging schedule structure defines a list of charging periods, as used in: [GetCompositeSchedule.conf](crate::server_init::get_composite_schedule) and [ChargingProfile]).
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedule {
    /// Optional. Duration of the charging schedule in seconds. If the duration is left empty, the last period will continue indefinitely or until end of the transaction in case startSchedule is absent.
    pub duration: Option<u32>,
    /// Optional. Starting point of an absolute schedule. If absent the schedule will be relative to start of charging.
    pub start_schedule: Option<DateTime<Utc>>,
    /// Required. The unit of measure Limit is expressed in.
    pub charging_rate_unit: ChargingRateUnit,
    /// Required. List of ChargingSchedulePeriod elements defining maximum power or current usage over time. The startSchedule of the first ChargingSchedulePeriod SHALL always be 0.
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    /// Optional. Minimum charging rate supported by the electric vehicle. The unit of measure is defined by the chargingRateUnit.
    /// This parameter is intended to be used by a local smart charging algorithm to optimize the power allocation for in the case a charging process is inefficient at lower charging rates. Accepts at most one digit fraction (e.g. 8.1)
    pub min_charging_rate: Option<f32>,
}

/// Charging schedule period structure defines a time period in a charging schedule, as used in: [ChargingSchedule].
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriod {
    /// Required. Start of the period, in seconds from the start of schedule. The value of StartPeriod also defines the stop time of the previous period.
    pub start_period: u32,
    /// 1 Required. Charging rate limit during the schedule period, in the applicable chargingRateUnit, for example in Amperes or Watts. Accepts at most one digit fraction (e.g. 8.1).
    pub limit: f32,
    /// Optional. The number of phases that can be used for charging. If a number of phases is needed, numberPhases=3 will be assumed unless another number is given.
    pub number_phases: Option<u32>,
}

/// Purpose of the charging profile, as used in: ChargingProfile.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
pub enum ChargingProfilePurpose {
    /// Configuration for the maximum power or current available for an entire Charge Point.
    ChargePointMaxProfile,
    /// Default profile *that can be configured in the Charge Point. When a new transaction is started, this profile SHALL be used,
    /// unless it was a transaction that was started by a RemoteStartTransaction.req with a ChargeProfile that is accepted by the Charge Point.
    TxDefaultProfile,
    /// Profile with constraints to be imposed by the Charge Point on the current transaction, or on a new transaction when this is started via a RemoteStartTransaction.req with a ChargeProfile.
    /// A profile with this purpose SHALL cease to be valid when the transaction terminates.
    TxProfile,
}

/// Kind of charging profile, as used in: ChargingProfile.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
pub enum ChargingProfileKind {
    /// Schedule periods are relative to a fixed point in time defined in the schedule
    Absolute,
    /// The schedule restarts periodically at the first schedule period.
    Recurring,
    /// Schedule periods are relative to a situation-specific start point (such as the start of a Transaction) that is determined by the charge point.
    Relative,
}

///Denotes whether a charge schedule recurs weekly or daily
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
pub enum RecurrencyKind {
    /// The schedule restarts every 24 hours, at the same time as in the startSchedule.
    Daily,
    /// The schedule restarts every 7 days, at the same time and day-of-the-week as in the startSchedule.
    Weekly,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
///Unit of power consumption in which a charging schedule is defined
pub enum ChargingRateUnit {
    ///Amperes per phase
    A,
    ///Total power in Watts
    W,
}

/// Typestate value for Id
pub struct Id(u32);
/// Typestate value for missing Id
pub struct NoId;

/// Typestate value for Level
pub struct Level(u32);
/// Typestate value for missing Level
pub struct NoLevel;

#[derive(Debug, Clone)]
/// Charging Profile Builder containing placeholder values to build into a charging profile
pub struct ChargingProfileBuilder<I, L> {
    /// Required. Unique identifier for this profile.
    pub charging_profile_id: I,
    /// Optional. Only valid if ChargingProfilePurpose is set to TxProfile, the transactionId MAY be used to match the profile to a specific transaction.
    pub transaction_id: Option<u32>,
    /// Required. Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values. Lowest level is 0.
    pub stack_level: L,
    /// Required. Defines the purpose of the schedule transferred by this message.
    pub charging_profile_purpose: ChargingProfilePurpose,
    /// Required. Indicates the kind of schedule.
    pub charging_profile_kind: ChargingProfileKind,
    /// Optional. Indicates the start point of a recurrence
    pub recurrency_kind: Option<RecurrencyKind>,
    /// Optional. Point in time at which the profile starts to be valid. If absent, the profile is valid as soon as it is received by the Charge Point.
    pub valid_from: Option<DateTime<Utc>>,
    /// Optional. Point in time at which the profile stops to be valid. If absent, the profile is valid until it is replaced by another profile.
    pub valid_to: Option<DateTime<Utc>>,
    /// Required. Contains limits for the available power or current over time
    pub charging_schedule: ChargingSchedule,
}

impl ChargingProfile {
    /// Create a new charging profile builder
    pub fn builder(charging_rate_unit: ChargingRateUnit) -> ChargingProfileBuilder<NoId, NoLevel> { ChargingProfileBuilder::new(charging_rate_unit) }
}

impl ChargingProfileBuilder<NoId, NoLevel> {
    /// Create new charging profile builder and return with initialized typestates
    /// Note, both profile purpose and kind are initialized for transactional purposes
    /// The builder will return a compile error if build is called when the Profile Id, Stack Level and Charging Schedule have not been set.
    ///
    /// A builder can be recycled to generate multiple variants of the same charging profile.
    pub fn new(charging_rate_unit: ChargingRateUnit) -> Self {
        let charging_schedule_period: Vec<ChargingSchedulePeriod> = Vec::new();
        let charging_schedule: ChargingSchedule = ChargingSchedule {
            duration: None,
            start_schedule: None,
            charging_rate_unit,
            charging_schedule_period,
            min_charging_rate: None,
        };
        ChargingProfileBuilder {
            charging_profile_id: NoId,
            transaction_id: None,
            stack_level: NoLevel,
            /// Default profile purpose
            charging_profile_purpose: ChargingProfilePurpose::TxProfile,
            /// Default profile kind
            charging_profile_kind: ChargingProfileKind::Relative,
            recurrency_kind: None,
            valid_from: None,
            valid_to: None,
            /// Charging schedule with empty period vec
            charging_schedule,
        }
    }

    /// QoL method for generating a basic TxProfile from scratch
    /// By default applies to the current transaction until that transaction finishes, and has no associated schedule.
    /// I.e. this will simply limit power for a transaction until it completes.
    pub fn new_tx_profile(self, limit: f32, id: u32, stack_level: u32) -> ChargingProfileBuilder<Id, Level> { self.purpose(ChargingProfilePurpose::TxProfile).id(id).stack_level(stack_level).add_period(0, limit, None) }
}

impl<I, L> ChargingProfileBuilder<I, L> {
    /// Add Id field and update typestate to verify it has been added
    pub fn id(self, charging_profile_id: u32) -> ChargingProfileBuilder<Id, L> {
        let Self {
            transaction_id,
            stack_level,
            charging_profile_purpose,
            charging_profile_kind,
            recurrency_kind,
            valid_from,
            valid_to,
            charging_schedule,
            ..
        } = self;
        ChargingProfileBuilder {
            charging_profile_id: Id(charging_profile_id),
            transaction_id,
            stack_level,
            charging_profile_purpose,
            charging_profile_kind,
            recurrency_kind,
            valid_from,
            valid_to,
            charging_schedule,
        }
    }

    /// Add Level field and update typestate to verify it has been added
    pub fn stack_level(self, stack_level: u32) -> ChargingProfileBuilder<I, Level> {
        let Self {
            charging_profile_id,
            transaction_id,
            charging_profile_purpose,
            charging_profile_kind,
            recurrency_kind,
            valid_from,
            valid_to,
            charging_schedule,
            ..
        } = self;
        ChargingProfileBuilder {
            charging_profile_id,
            transaction_id,
            stack_level: Level(stack_level),
            charging_profile_purpose,
            charging_profile_kind,
            recurrency_kind,
            valid_from,
            valid_to,
            charging_schedule,
        }
    }

    /// Add period to periods vector
    pub fn add_period(mut self, start_period: u32, limit: f32, number_phases: Option<u32>) -> Self {
        self.charging_schedule.charging_schedule_period.push(ChargingSchedulePeriod { start_period, limit, number_phases });
        self
    }

    /// Remove all periods from charging profile builder
    pub fn clear_periods(mut self) -> Self {
        self.charging_schedule.charging_schedule_period.clear();
        self
    }

    /// Add transaction_id field
    pub fn schedule_duration(mut self, duration: u32) -> Self {
        self.charging_schedule.duration = Some(duration);
        self
    }

    /// Add start_schedule field
    pub fn schedule_start(mut self, start_schedule: DateTime<Utc>) -> Self {
        self.charging_schedule.start_schedule = Some(start_schedule);
        self
    }

    /// Add charging_rate_unit field
    pub fn schedule_charging_rate_unit(mut self, charging_rate_unit: ChargingRateUnit) -> Self {
        self.charging_schedule.charging_rate_unit = charging_rate_unit;
        self
    }

    /// Add min_charging_rate field
    pub fn schedule_min_charging_rate(mut self, min_charging_rate: f32) -> Self {
        self.charging_schedule.min_charging_rate = Some(min_charging_rate);
        self
    }

    /// transaction_id field
    pub fn transaction_id(mut self, transaction_id: u32) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Add charging_profile_purpose field
    pub fn purpose(mut self, charging_profile_purpose: ChargingProfilePurpose) -> Self {
        self.charging_profile_purpose = charging_profile_purpose;
        self
    }

    /// Add charging_profile_kind field
    pub fn kind(mut self, charging_profile_kind: ChargingProfileKind) -> Self {
        self.charging_profile_kind = charging_profile_kind;
        self
    }

    /// Add recurrency_kind field
    pub fn recurrency_kind(mut self, recurrency_kind: RecurrencyKind) -> Self {
        self.recurrency_kind = Some(recurrency_kind);
        self
    }

    /// Add valid_from field
    pub fn valid_from(mut self, valid_from: DateTime<Utc>) -> Self {
        self.valid_from = Some(valid_from);
        self
    }

    /// Add valid_to field
    pub fn valid_to(mut self, valid_to: DateTime<Utc>) -> Self {
        self.valid_to = Some(valid_to);
        self
    }
}

impl ChargingProfileBuilder<Id, Level> {
    /// Build ChargingProfile from existing builder struct
    /// This function CANNOT be called unless the profile ID, Stack Level and Schedule have been called
    pub fn build(self) -> ChargingProfile {
        let Id(charging_profile_id) = self.charging_profile_id;
        let Level(stack_level) = self.stack_level;

        ChargingProfile {
            charging_profile_id,
            transaction_id: self.transaction_id,
            stack_level,
            charging_profile_purpose: self.charging_profile_purpose,
            charging_profile_kind: self.charging_profile_kind,
            recurrency_kind: self.recurrency_kind,
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            charging_schedule: self.charging_schedule,
        }
    }
}
