//! A collection of common types shared by multiple messages.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
///Unit of power consumption in which a charging schedule is defined
pub enum ChargingRateUnit {
    ///Amperes per phase 
    A,
    ///Total power in Watts
    W,
}

///Denotes whether a charge schedule recurs weekly or daily
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum RecurrencyKind {
    Daily,
    Weekly,
}

///Generic status message denoting Accepted or Rejected state.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SimpleStatus {
    Accepted,
    Rejected,
}

//START Value Field
///Single sampled value, used by [MeterValues](crate::point_init::MeterValues)
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    /// Required. Value as a “Raw” (decimal) number or “SignedData”. Field Type is “string” to allow for digitally signed data readings. Decimal numeric values are also acceptable to allow fractional values for measurands such as Temperature and Current.
    pub value: String,
    /// Optional. Type of detail value: start, end or sample. Default = “Sample.Periodic”
    pub context: Option<SampledContext>,
    /// Optional. Raw or signed data. Default = “Raw”
    pub format: Option<SampledFormat>,
    /// Optional. Type of measurement. Default = “Energy.Active.Import.Register”
    pub measurant: Option<SampledMeasurand>,
    /// Optional. indicates how the measured value is to be interpreted. For instance between L1 and neutral (L1-N) Please note that not all values of phase are applicable to all Measurands. When phase is absent, the measured value is interpreted as an overall value.
    pub phase: Option<SampledPhase>,
    /// Optional. Location of measurement. Default=”Outlet”
    pub location: Option<SampledLocation>,
    /// Optional. Unit of the value. Default = “Wh” if the (default) measurand is an “Energy” type.
    pub unit: Option<SampledUnit>,
}

/// Values of the context field of a value in SampledValue.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SampledContext {
    /// Value taken at start of interruption.
    #[serde(rename = "Interruption.Begin")]
    /// Value taken when resuming after interruption.
    InterruptionBegin,
    #[serde(rename = "Interruption.End")]
    ///Value taken when resuming after interruption.
    InterruptionEnd,
    #[serde(rename = "Sample.Clock")]
    /// Value taken at clock aligned interval.
    SampleClock,
    #[serde(rename = "Sample.Periodic")]
    /// Value taken as periodic sample relative to start time of transaction.
    SamplePeriodic,
    #[serde(rename = "Transaction.Begin")]
    /// Value taken at start of transaction.
    TransactionBegin,
    #[serde(rename = "Transaction.End")]
    /// Value taken at end of transaction.
    TransactionEnd,
    /// Value taken in response to a TriggerMessage.req
    Trigger,
    /// Value for any other situations.
    Other,
}

/// Format that specifies how the value element in SampledValue is to be interpreted.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SampledFormat {
    Raw,
    SignedData,
}

/// Allowable values of the optional "measurand" field of a Value element, as used in MeterValuesRequest and StopTransaction.req messages. Default value of "measurand" is always "Energy.Active.Import.Register"
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SampledMeasurand {
    /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    /// Numerical value read from the "reactive electrical energy" (VARh or kVARh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    /// Numerical value read from the "reactive electrical energy" (VARh or kVARh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    /// Absolute amount of "active electrical energy" (Wh or kWh) exported (to the grid) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration 
    /// configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    /// Absolute amount of "active electrical energy" (Wh or kWh) imported (from the grid supply) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval 
    /// duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    /// Absolute amount of "reactive electrical energy" (VARh or kVARh) exported (to the grid) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval 
    /// duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    /// Absolute amount of "reactive electrical energy" (VARh or kVARh) imported (from the grid supply) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable 
    /// interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    /// Instantaneous active power exported by EV. (W or kW)
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    /// Instantaneous active power imported by EV. (W or kW)
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    /// Maximum power offered to EV
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    /// Instantaneous reactive power exported by EV. (var or kvar)
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    /// Instantaneous reactive power imported by EV. (var or kvar)
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    /// Instantaneous power factor of total energy flow
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    /// Instantaneous current flow to EV
    #[serde(rename = "Current.Import")]
    CurrentImport,
    ///Instantaneous current flow from EV
    #[serde(rename = "Current.Export")]
    CurrentExport,
    /// Maximum current offered to EV
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    /// Instantaneous AC RMS supply voltage
    Voltage,
    /// Instantaneous reading of powerline frequency. NOTE: OCPP 1.6 does not have a UnitOfMeasure for frequency, the UnitOfMeasure for any SampledValue with measurand: Frequency is Hertz.
    Frequency,
    /// Temperature reading inside Charge Point.
    Temperature,
    /// State of charge of charging vehicle in percentage
    SoC,
    /// Fan speed in RPM
    RPM,
}

/// Phase as used in SampledValue. Phase specifies how a measured value is to be interpreted. Please note that not all values of Phase are applicable to all Measurands.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SampledPhase {
    L1,
    L2,
    L3,
    N,
    #[serde(rename = "L1-N")]
    L1N,
    #[serde(rename = "L2-N")]
    L2N,
    #[serde(rename = "L3-N")]
    L3N,
    #[serde(rename = "L1-L2")]
    L1L2,
    #[serde(rename = "L2-L3")]
    L2L3,
    #[serde(rename = "L3-L1")]
    L3L1,
}

/// Allowable values of the optional "location" field of a value element in SampledValue.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum SampledLocation {
    /// Measurement inside body of Charge Point (e.g. Temperature)
    Body,
    ///Measurement taken from cable between EV and Charge Point
    Cable,
    ///Measurement taken by EV
    EV,
    ///Measurement at network (“grid”) inlet connection
    Inlet,
    ///Measurement at a Connector. Default value
    Outlet,
}

/// Allowable values of the optional "unit" field of a Value element, as used in SampledValue. Default value of "unit" is always "Wh".
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
#[warn(non_camel_case_types)]
pub enum SampledUnit {
    /// Watt-hours (energy). Default.
    Wh,
    /// kiloWatt-hours (energy).
    #[serde(rename = "kWh")]
    KWh,
    /// Var-hours (reactive energy).
    #[serde(rename = "varh")]
    Varh,
    /// kilovar-hours (reactive energy).
    #[serde(rename = "kvarh")]
    Kvarh,
    /// Watts (power).
    W,
    /// kilowatts (power).
    #[serde(rename = "kW")]
    Kw,
    /// VoltAmpere (apparent power).
    #[serde(rename = "VA")]
    Va,
    /// kiloVolt Ampere (apparent power).
    #[serde(rename = "kVA")]
    Kva,
    /// Vars (reactive power).
    #[serde(rename = "var")]
    Var,
    /// kilovars (reactive power).
    #[serde(rename = "kvar")]
    Kvar,
    /// Amperes (current).
    A,
    /// Voltage (r.m.s. AC).
    V,
    /// Degrees (temperature).
    Celsius,
    /// Degrees (temperature).
    Fahrenheit,
    /// Degrees Kelvin (temperature).
    K,
    /// Percentage.
    Percent,
}
//END Value Field

//START Profile Field
/// A ChargingProfile consists of a ChargingSchedule, describing the amount of power or current that can be delivered per time interval.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
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

/// Charging schedule structure defines a list of charging periods, as used in: [GetCompositeSchedule.conf](crate::server_init::get_composite_schedule) and [ChargingProfile].
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum ChargingProfileKind {
    /// Schedule periods are relative to a fixed point in time defined in the schedule
    Absolute,
    /// The schedule restarts periodically at the first schedule period.
    Recurring,
    /// Schedule periods are relative to a situation-specific start point (such as the start of a Transaction) that is determined by the charge point.
    Relative,
}
//END Profile Field

//START ID Tag Field
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    expiry_date: DateTime<Utc>,
    parent_id_tag: String,
    status: AuthorizationStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display)]
pub enum AuthorizationStatus {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}
//END ID Tag FIeld
