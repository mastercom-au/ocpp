//! Definition and builder for the Charge Profile structure, used to set charging behaviour and scheduling
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

/// A ChargingProfile consists of a ChargingSchedule, describing the amount of power or current that can be delivered per time interval.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl ChargingSchedule {
    /// Builder constructor to generate a default new charging schedule
    pub fn new(charging_rate_unit: ChargingRateUnit, charging_schedule_period: Vec<ChargingSchedulePeriod>) -> Self {
        Self {
            duration: None,
            start_schedule: None,
            charging_rate_unit,
            charging_schedule_period,
            min_charging_rate: None,
        }
    }

    /// Set Duration
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set Duration
    pub fn start_schedule(mut self, start_schedule: DateTime<Utc>) -> Self {
        self.start_schedule = Some(start_schedule);
        self
    }

    /// Set Duration
    pub fn charging_rate_unit(mut self, charging_rate_unit: ChargingRateUnit) -> Self {
        self.charging_rate_unit = charging_rate_unit;
        self
    }

    /// Set Duration
    pub fn charging_schedule_period(mut self, charging_schedule_period: Vec<ChargingSchedulePeriod>) -> Self {
        self.charging_schedule_period = charging_schedule_period;
        self
    }

    /// Set Duration
    pub fn min_charging_rate(mut self, min_charging_rate: f32) -> Self {
        self.min_charging_rate = Some(min_charging_rate);
        self
    }
}

impl ChargingProfile {
    /// Builder constructor to generate a default new charging profile
    pub fn new(charging_profile_id: u32, charging_schedule: ChargingSchedule) -> Self {
        Self {
            charging_profile_id,
            transaction_id: None,
            // Default
            stack_level: 10,
            // Default
            charging_profile_purpose: ChargingProfilePurpose::TxProfile,
            // Default
            charging_profile_kind: ChargingProfileKind::Relative,
            recurrency_kind: None,
            valid_from: None,
            valid_to: None,
            charging_schedule,
        }
    }

    /// Set Charging Profile Id
    pub fn charging_profile_id(mut self, charging_profile_id: u32) -> Self {
        self.charging_profile_id = charging_profile_id;
        self
    }

    /// Set Transaction Id
    pub fn transaction_id(mut self, transaction_id: u32) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Set Stack Level
    pub fn stack_level(mut self, stack_level: u32) -> Self {
        self.stack_level = stack_level;
        self
    }

    /// Set Charging Profile Purpose
    pub fn charging_profile_purpose(mut self, charging_profile_purpose: ChargingProfilePurpose) -> Self {
        self.charging_profile_purpose = charging_profile_purpose;
        self
    }

    /// Set Charging Profile Kind
    pub fn charging_profile_kind(mut self, charging_profile_kind: ChargingProfileKind) -> Self {
        self.charging_profile_kind = charging_profile_kind;
        self
    }

    /// Set Recurrency Kind
    pub fn recurrency_kind(mut self, recurrency_kind: RecurrencyKind) -> Self {
        self.recurrency_kind = Some(recurrency_kind);
        self
    }

    /// Set Valid-From
    pub fn valid_from(mut self, valid_from: DateTime<Utc>) -> Self {
        self.valid_from = Some(valid_from);
        self
    }

    /// Set Valid-To
    pub fn valid_to(mut self, valid_to: DateTime<Utc>) -> Self {
        self.valid_to = Some(valid_to);
        self
    }

    /// Set Charging Schedule
    pub fn charging_schedule(mut self, charging_schedule: ChargingSchedule) -> Self {
        self.charging_schedule = charging_schedule;
        self
    }
}
