use crate::common_types::SampledValue;
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
        value string
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
    pub id_tag: Option<String>,
    pub meter_stop: u32,
    pub time_stamp: DateTime<Utc>,
    pub transaction_id: u32,
    pub reason: Option<StopReason>,
    pub transaction_data: Option<Vec<StopTransactionData>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionData {
    pub time_stamp: DateTime<Utc>,
    pub sampled_value: Vec<SampledValue>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum StopReason {
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

// -------------------------- RESPONSE --------------------------
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionResponse {
    pub id_tag_info: Option<StopIdTagInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopIdTagInfo {
    pub expiry_date: Option<DateTime<Utc>>,
    pub parent_id_tag: Option<String>,
    pub status: StopTransactionStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StopTransactionStatus {
    Accepted,
    Rejected,
    Expired,
    Invalid,
    ConcurrentTx,
}
