//! Server request to set a charging schedule for a ChargePoint
//!
//! A Central System can send a SetChargingProfile.req to a Charge Point, to set a charging profile, in the following situations:
//! 1. At the start of a transaction to set the charging profile for the transaction;
//! 2. In a RemoteStartTransaction.req sent to a Charge Point
//! 3. During a transaction to change the active profile for the transaction;
//! 4. Outside the context of a transaction as a separate message to set a charging profile to a local controller, Charge Point, or a default charging profile to a connector.
//!
//! # 1. Setting a charging profile at start of transaction
//! If the Central System receives a StartTransaction.req the Central System SHALL respond with a StartTransaction.conf. If there is a need for a charging profile, The Central System MAY choose to send a SetChargingProfile.req to the Charge Point.
//!
//! It is RECOMMENDED to check the timestamp in the StartTransaction.req PDU prior to sending a charging profile to check if the transaction is likely to be still ongoing. The StartTransaction.req might have been cached during an offline period.
//!
//! # 2. Setting a charge profile in a RemoteStartTransaction request
//! The Central System MAY include a charging profile in a RemoteStartTransaction request.
//! If the Central System includes a ChargingProfile, the ChargingProfilePurpose MUST be set to TxProfile and the transactionId SHALL NOT be set.
//!
//! The Charge Point SHALL apply the given profile to the newly started transaction. This transaction will get a transactionId assigned by Central System via a StartTransaction.conf.
//!
//! When the Charge Point receives a SetChargingProfile.req, with the transactionId for this transaction, with the same StackLevel as the profile given in the RemoteStartTransaction.req, the Charge Point SHALL replace the existing charging profile, otherwise it SHALL install/stack the profile next to the already existing profile(s).
//!
//! # 3. Setting a charging profile during a transaction.
//! The Central System MAY send a charging profile to a Charge Point to update the charging profile for that transaction. The Central System SHALL use the SetChargingProfile.req PDU for that purpose. If a charging profile with the same chargingProfileId, or the same combination of stackLevel / ChargingProfilePurpose, exists on the Charge Point, the new charging profile SHALL replace the existing charging profile, otherwise it SHALL be added. The Charge Point SHALL then re-evaluate its collection of charge profiles to determine which charging profile will become active. In order to ensure that the updated charging profile applies only to the current transaction, the chargingProfilePurpose of the ChargingProfile MUST be set to TxProfile. (See section: Charging Profile Purposes)
//!
//! # 4. Setting a charging profile outside of a transaction
//! The Central System MAY send charging profiles to a Charge Point that are to be used as default charging profiles.
//! The Central System SHALL use the SetChargingProfile.req PDU for that purpose. Such charging profiles MAY be sent at any time. If a charging profile with the same chargingProfileId, or the same combination of stackLevel ChargingProfilePurpose, exists on the Charge Point, the new charging profile SHALL replace the existing charging profile, otherwise it SHALL be added. The Charge Point SHALL then re-evaluate its collection of charge profiles to determine which charging profile will become active.
//!
//! # Note
//!
//! * To prevent mismatch between transactions and a TxProfile, The Central System SHALL include the transactionId in a SetChargingProfile.req if the profile applies to a specific transaction.
//!
//! * It is not possible to set a ChargingProfile with purpose set to TxProfile without presence of an active transaction, or in advance of a transaction.
//!
//! * When a ChargingProfile is refreshed during execution, it is advised to put the startSchedule of the new ChargingProfile in the past, so there is no period of default charging behaviour inbetween the ChargingProfiles. The Charge Point SHALL continue to execute the existing ChargingProfile until the new ChargingProfile is installed.
//!
//! * If the chargingSchedulePeriod is longer than duration, the remainder periods SHALL not be executed. If duration is longer than the chargingSchedulePeriod, the Charge Point SHALL keep the value of the last chargingSchedulePeriod until duration has ended.
//!
//! * When recurrencyKind is used in combination with a chargingSchedulePeriod and/or duration that is longer then the recurrence period duration, the remainder periods SHALL not be executed.
//!
//! * The StartSchedule of the first chargingSchedulePeriod in a chargingSchedule SHALL always be 0
//!
//! * When recurrencyKind is used in combination with a chargingSchedule duration shorter than the recurrencyKind period, the Charge Point SHALL fall back to default behaviour after the chargingSchedule duration ends.
//! This fall back means that the Charge Point SHALL use a ChargingProfile with a lower stackLevel if available.
//! If no other ChargingProfile is available, the Charge Point SHALL allow charging as if no ChargingProfile is installed.
//! If the chargingSchedulePeriod and/or duration is longer then the recurrence period duration, the remainder periods SHALL not be executed.

use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

pub use crate::ChargingProfile;

/* Structure
ConnectorId u32
csChargingProfile struct
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
#[json_validate("../json_schemas/SetChargingProfile.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the SetChargingProfile.req PDU sent by the Central System to the Charge Point.
/// The Central System uses this message to send charging profiles to a Charge Point.
pub struct SetChargingProfileRequest {
    /// Required. The connector to which the charging profile applies. If connectorId = 0, the message contains an overall limit for the Charge Point.
    pub connector_id: u32,
    /// Required. The charging profile to be set at the Charge Point.
    pub cs_charging_profiles: ChargingProfile,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/SetChargingProfileResponse.json")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Field definition of the SetChargingProfile.conf PDU sent by the Charge Point to the Central System in response to a SetChargingProfile.req PDU.
pub struct SetChargingProfileResponse {
    /// Required. Returns whether the Charge Point has been able to process the message successfully. This does not guarantee the
    /// schedule will be followed to the letter. There might be other constraints the Charge Point may need to take into account.
    pub status: ChargingProfileStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
/// Status returned in response to SetChargingProfile.req.
pub enum ChargingProfileStatus {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// Charge Point indicates that the request is not supported.
    NotSupported,
}
