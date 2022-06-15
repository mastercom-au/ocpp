//! Definition for the meter value type
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::{Display, EnumIter};

/// Collection of one or more sampled values (as seen in [MeterValues.req](crate::point_init::meter_values) and [StopTransaction.req](crate::point_init::stop_transaction)), all sampled at the same time.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue {
    /// Required. Timestamp for measured value(s).
    pub timestamp: DateTime<Utc>,
    /// Required. One or more measured values
    pub sampled_value: Vec<SampledValue>,
}

///Single sampled value, used by [MeterValues](crate::point_init::meter_values)
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    /// Required. Value as a “Raw” (decimal) number or “SignedData”. Field Type is “string” to allow for digitally signed data readings. Decimal numeric values are also acceptable to allow fractional values for measurands such as Temperature and Current.
    pub value: String,
    /// Optional. Type of detail value: start, end or sample. Default = “Sample.Periodic”
    pub context: Option<SampledContext>,
    /// Optional. Raw or signed data. Default = “Raw”
    pub format: Option<SampledFormat>,
    /// Optional. Type of measurement. Default = “Energy.Active.Import.Register”
    pub measurand: Option<SampledMeasurand>,
    /// Optional. indicates how the measured value is to be interpreted. For instance between L1 and neutral (L1-N) Please note that not all values of phase are applicable to all Measurands. When phase is absent, the measured value is interpreted as an overall value.
    pub phase: Option<SampledPhase>,
    /// Optional. Location of measurement. Default=”Outlet”
    pub location: Option<SampledLocation>,
    /// Optional. Unit of the value. Default = “Wh” if the (default) measurand is an “Energy” type.
    pub unit: Option<SampledUnit>,
}

/// Values of the context field of a value in SampledValue.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
pub enum SampledFormat {
    /// Data is to be interpreted as integer/decimal numeric data.
    Raw,
    /// Data is represented as a signed binary data block, encoded as hex data.
    SignedData,
}

/// Allowable values of the optional "measurand" field of a Value element, as used in MeterValuesRequest and StopTransaction.req messages. Default value of "measurand" is always "Energy.Active.Import.Register"
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone, EnumIter)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
pub enum SampledPhase {
    /// Measured on L1
    L1,
    /// Measured on L2
    L2,
    /// Measured on L3
    L3,
    /// Measured on Neutral
    N,
    /// Measured on L1 with respect to Neutral conductor
    #[serde(rename = "L1-N")]
    L1N,
    /// Measured on L2 with respect to Neutral conductor
    #[serde(rename = "L2-N")]
    L2N,
    /// Measured on L3 with respect to Neutral conductor
    #[serde(rename = "L3-N")]
    L3N,
    /// Measured between L1 and L2
    #[serde(rename = "L1-L2")]
    L1L2,
    /// Measured between L2 and L3
    #[serde(rename = "L2-L3")]
    L2L3,
    /// Measured between L3 and L1
    #[serde(rename = "L3-L1")]
    L3L1,
}

/// Allowable values of the optional "location" field of a value element in SampledValue.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
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
