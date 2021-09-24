//#[macro_use]
use chrono::DateTime;
use chrono::Utc;
use serde;
//use jsonschema;
use serde::{Deserialize, Serialize};
//use serde_json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Authorize {
    pub id_tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BootNotification {
    pub charge_point_vendor: String,
    pub charge_point_model: String,
    pub charge_point_serial_number: Option<String>,
    pub charge_box_serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub meter_type: Option<String>,
    pub meter_serial_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailability {
    pub connector_id: u32,
    pub r#type: ChangeAvailabilityType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChangeAvailabilityType {
    Inoperative,
    Operative,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfiguration {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClearCache {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTransfer {
    pub vendor_id: String,
    pub message_id: Option<String>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConfiguration {
    pub key: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {}

//---------------------------------------------METER VALUE-----------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeterValues {
    pub connector_id: u32,
    pub transaction_id: u32,
    pub meter_value: Vec<MVValues>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MVValues {
    pub timestamp: DateTime<Utc>,
    pub sampled_value: Vec<MVSampledValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MVSampledValue {
    pub value: String,
    pub context: MVSampledContext,
    pub format: MVSampledFormat,
    pub measurant: MVSampledMeasurand,
    pub phase: MVSampledPhase,
    pub location: MVSampledLocation,
    pub unit: MVSampledUnit,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MVSampledContext {
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
pub enum MVSampledFormat {
    Raw,
    SignedData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MVSampledMeasurand {
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
pub enum MVSampledPhase {
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
pub enum MVSampledLocation {
    Cable,
    EV,
    Inlet,
    Outlet,
    Body,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[warn(non_camel_case_types)]
pub enum MVSampledUnit {
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
//-----------------------------------------END METER VALUE-----------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransaction {
    //complex
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransaction {
    pub transaction_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reset {
    //complex
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartTransaction {
    pub connector_id: u32,
    pub id_tag: String,
    pub meter_start: i32,
    pub reservation_id: Option<i32>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotification {
    pub connector_id: u32,
    error_code: String,
    info: Option<String>,
    status: String,
    timestamp: Option<String>,
    vendor_id: Option<String>,
    vendor_error_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopTransaction {
    //complex
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnector {
    pub connector_id: u32,
}
