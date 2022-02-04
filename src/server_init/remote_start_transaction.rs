//! Server request to start a transaction (Can include a charge profile)
//!
//! # Behaviour
//! The following typical use cases are the reason for Remote Start Transaction:
//! * Enable a CPO operator to help an EV driver that has problems starting a transaction.
//! * Enable mobile apps to control charging transactions via the Central System.
//! * Enable the use of SMS to control charging transactions via the Central System.
//!
//! The RemoteStartTransaction.req SHALL contain an identifier (idTag), which Charge Point SHALL use, if it is able to start a transaction,
//! to send a StartTransaction.req to Central System. The transaction is started in the same way as described in StartTransaction. The
//! RemoteStartTransaction.req MAY contain a connector id if the transaction is to be started on a specific connector. When no connector
//! id is provided, the Charge Point is in control of the connector selection. A Charge Point MAY reject a RemoteStartTransaction.req without a connector id.
//!
//! The Central System MAY include a ChargingProfile in the RemoteStartTransaction request. The purpose of this ChargingProfile SHALL be set
//! to TxProfile. If accepted, the Charge Point SHALL use this ChargingProfile for the transaction. If a Charge Point without support for Smart
//! Charging receives a RemoteStartTransaction.req with a Charging Profile, this parameter SHOULD be ignored.
//!
//! # Response
//! Upon receipt, the Charge Point SHALL reply with RemoteStartTransaction.conf and a status indicating whether it has accepted the request and will attempt to start a transaction.
//! The effect of the RemoteStartTransaction.req message depends on the value of the AuthorizeRemoteTxRequests configuration key in the Charge Point.
//!
//! * If the value of AuthorizeRemoteTxRequests is true, the Charge Point SHALL behave as if in response to a local action at the Charge Point
//! to start a transaction with the idTag given in the RemoteStartTransaction.req message. This means that the Charge Point will first try to
//! authorize the idTag, using the Local Authorization List, Authorization Cache and/or an Authorize.req request. A transaction will only be
//! started after authorization was obtained.
//! * If the value of AuthorizeRemoteTxRequests is false, the Charge Point SHALL immediately try to start a transaction for the idTag given in
//! the RemoteStartTransaction.req message. Note that after the transaction has been started, the Charge Point will send a StartTransaction
//! request to the Central System, and the Central System will check the authorization status of the idTag when processing this StartTransaction request.
//!

pub use crate::common_types::{ChargingProfile, SimpleStatus};
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
/// Field definitions of the RemoteStartTransaction.req PDU sent to Charge Point by Central System.
pub struct RemoteStartTransactionRequest {
    /// Optional. Number of the connector on which to start the transaction. connectorId SHALL be > 0
    pub connector_id: Option<u32>,
    /// Required. The identifier that Charge Point must use to start a transaction.
    pub id_tag: String,
    /// Optional. Charging Profile to be used by the Charge Point for the requested transaction. ChargingProfilePurpose MUST be set to TxProfile
    pub charging_profile: Option<ChargingProfile>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/RemoteStartTransactionResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definitions of the RemoteStartTransaction.conf PDU sent from Charge Point to Central System.
pub struct RemoteStartTransactionResponse {
    /// Required. Status indicating whether Charge Point accepts the request to start a transaction.
    pub status: SimpleStatus,
}
