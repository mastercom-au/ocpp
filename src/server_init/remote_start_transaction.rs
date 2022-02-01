use crate::common_types::{ChargingProfile, SimpleStatus};
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/* Structure
ConnectorId u32
idTag String
chargingProfile struct
    chargingProfileId u32
    transactionId u32
    stackLevel u32
    chargingProfilePurpose String Enum
    chargingprofileKind String Enum
    recurrencyKind String Enum
    validFrom datetime<utc>
    validTo datetime<utc>
    chargingSchedule struct
        duration u32
        startSchedule date<utc>
        chargingRateUnit String enum
        minChargingRate f32 (increment 0.1)
        chargingSchedulePeriod Vec<obj>
            startPeriod u32
            limit f32 (increment 0.1)
            numberPhases u32
*/

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/RemoteStartTransaction.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransactionRequest {
    pub connector_id: Option<u32>,
    pub id_tag: String,
    pub charging_profile: Option<ChargingProfile>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/RemoteStartTransactionResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransactionResponse {
    pub status: SimpleStatus,
}
