use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*Structure
idTag String
meterStop u32
timestamp dateTime<utc>
transactionId u32,
reason enum String,
transactionData vec<obj>
    timeStamp dateTime<utc>
    sampledValue vec<obj>
        context enum String
        format enum String
        measurand enum String
        phase enum String
        location enum String
        unit enum String
*/

// -------------------------- REQUEST ---------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionRequest {
    pub id_tag: String,
    pub meter_stop: u32,
    pub time_stamp: DateTime<Utc>,
    pub transaction_id: u32,
    pub reason: Reason,
    pub transaction_data: Vec<TransactionData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub time_stamp: DateTime<Utc>,
    pub sampled_value: Vec<SampledValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    pub value: String,
    pub context: SampledContext,
    pub format: SampledFormat,
    pub measurant: SampledMeasurand,
    pub phase: SampledPhase,
    pub location: SampledLocation,
    pub unit: SampledUnit,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Reason {
    EmergencyStop,
    EVDisconnected,
    HardReset,
    Local,
    Other,
    PowerLoss,
    Reboot,
    Remote,
    SoftReset,
    UnlockCommand,
    DeAuthorized,
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
    Fahrenheit,
    Percent,
}

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionResponse {
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    pub expiry_date: Option<DateTime<Utc>>,
    pub parent_id_tag: Option<String>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Accepted,
    Rejected,
    Expired,
    Invalid,
    ConcurrentTx,
}