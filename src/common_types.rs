use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChargingRateUnit {
    A,
    W,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum RecurrencyKind {
    Daily,
    Weekly,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SimpleStatus {
    Accepted,
    Rejected,
}

//START Value Field
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    pub value: String,
    pub context: Option<SampledContext>,
    pub format: Option<SampledFormat>,
    pub measurant: Option<SampledMeasurand>,
    pub phase: Option<SampledPhase>,
    pub location: Option<SampledLocation>,
    pub unit: Option<SampledUnit>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SampledContext {
    #[serde(rename = "Interruption.Begin")]
    InterruptionBegin,
    #[serde(rename = "Interruption.End")]
    InterruptionEnd,
    #[serde(rename = "Sample.Clock")]
    SampleClock,
    #[serde(rename = "Sample.Periodic")]
    SamplePeriodic,
    #[serde(rename = "Transaction.Begin")]
    TransactionBegin,
    #[serde(rename = "Transaction.End")]
    TransactionEnd,
    Trigger,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SampledFormat {
    Raw,
    SignedData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SampledMeasurand {
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    #[serde(rename = "Current.Import")]
    CurrentImport,
    #[serde(rename = "Current.Export")]
    CurrentExport,
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    Voltage,
    Frequency,
    Temperature,
    SoC,
    RPM,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SampledLocation {
    Cable,
    EV,
    Inlet,
    Outlet,
    Body,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[warn(non_camel_case_types)]
pub enum SampledUnit {
    #[serde(rename = "Wh")]
    WH,
    #[serde(rename = "kWh")]
    KWh,
    #[serde(rename = "varh")]
    Varh,
    #[serde(rename = "kvarh")]
    KVarh,
    #[serde(rename = "W")]
    W,
    #[serde(rename = "kW")]
    KW,
    VA,
    KVA,
    #[serde(rename = "var")]
    Var,
    #[serde(rename = "kvar")]
    KVar,
    A,
    V,
    K,
    Celsius,
    Celcius,
    Fahrenheit,
    Percent,
}
//END Value Field

//START Profile Field
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfile {
    charging_profile_id: u32,
    transaction_id: Option<u32>,
    stack_level: u32,
    charging_profile_purpose: ChargingProfilePurpose,
    charging_profile_kind: ChargingProfileKind,
    recurrency_kind: Option<RecurrencyKind>,
    valid_from: Option<DateTime<Utc>>,
    valid_to: Option<DateTime<Utc>>,
    charging_schedule: ProfileSchedule,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSchedule {
    pub duration: Option<u32>,
    pub start_schedule: Option<DateTime<Utc>>,
    pub charging_rate_unit: ChargingRateUnit,
    pub charging_schedule_period: Vec<ProfileSchedulePeriod>,
    pub min_charging_rate: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSchedulePeriod {
    pub start_period: u32,
    pub limit: f32, //increment 0.1
    pub number_phases: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChargingProfileKind {
    Absolute,
    Recurring,
    Relative,
}
//END Profile Field