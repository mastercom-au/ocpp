//! A set of sampled data from the charge point denoting a series of measurements over a period of time.
//!
//! # Behaviour
//! It is up to the Charge Point to decide when it will send meter values. This can be configured using the ChangeConfiguration.req message to data
//! acquisition intervals and specify data to be acquired & reported.The Charge Point SHALL send a MeterValues.req PDU for offloading meter values.
//! The request PDU SHALL contain for each sample:
//!
//! 1. The id of the Connector from which samples were taken. If the connectorId is 0, it is associated with the entire Charge Point. If the connectorId
//!  is 0 and the Measurand is energy related, the sample SHOULD be taken from the main energy meter.
//! 2. The transactionId of the transaction to which these values are related, if applicable. If there is no transaction in progress or if the values are
//!  taken from the main meter, then transaction id may be omitted.
//! 3. One or more meterValue elements, of type MeterValue, each representing a set of one or more data values taken at a particular point in time.
//!
//! Each MeterValue element contains a timestamp and a set of one or more individual sampledvalue elements, all captured at the same point in time.
//!
//! Each sampledValue element contains a single value datum. The nature of each sampledValue is determined by the optional measurand, context, location, unit, phase, and format fields.
//!
//! Two measurands (Current.Offered and Power.Offered) are available that are strictly speaking no measured values. They indicate the maximum amount of current/power that is being offered to the EV and are intended for use in smart charging applications.
//!
//! For individual connector phase rotation information, the Central System MAY query the ConnectorPhaseRotation configuration key on the Charging Point via GetConfiguration. The Charge Point SHALL report the phase rotation in respect to the grid connection.
//! Possible values per connector are: 36NotApplicable, Unknown, RST, RTS, SRT, STR, TRS and TSR. see section Standard Configuration Key Names & Values for more information.
//!
//! The EXPERIMENTAL optional format field specifies whether the data is represented in the normal (default) form as a simple numeric value ("Raw"), or as “SignedData”, an opaque digitally signed binary data block, represented as hex data.
//! This experimental field may be deprecated and subsequently removed in later versions, when a more mature solution alternative is provided.
//!
//! To retain backward compatibility, the default values of all of the optional fields on a sampledValue element are such that a value without any additional fields will be interpreted, as a register reading of active import energy in Wh (Watt-hour) units.
//! # Response
//! Upon receipt of a MeterValues.req PDU, the Central System SHALL respond with a MeterValues.conf.
//! It is likely that The Central System applies sanity checks to the data contained in a MeterValues.req it received.
//! The outcome of such sanity checks SHOULD NOT ever cause the Central System to not respond with a MeterValues.conf. Failing to respond with a MeterValues.conf will only cause the Charge Point to try the same message again as specified in Error responses to transaction-related messages.

pub use crate::common_types::MeterValue;
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

/* Structure
connectorId u32
transactonId u32
meterValue Vec<obj>
    timestamp dateTime<utc>
    sampledValue Vec<obj>
        value String
        context enum String
        format enum String
        measurand enum String
        phase enum String
        location enum String
        unit enum String
*/

// -------------------------- REQUEST ---------------------------
#[json_validate("../json_schemas/MeterValues.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the MeterValues.req PDU sent by the Charge Point to the Central System.
pub struct MeterValuesRequest {
    /// Required. This contains a number (>0) designating a connector of the ChargePoint.‘0’ (zero) is used to designate the main powermeter.
    pub connector_id: u32,
    /// Optional. The transaction to which these meter samples are related.
    pub transaction_id: Option<u32>,
    /// Required. The sampled meter values with timestamps.
    pub meter_value: Vec<MeterValue>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/MeterValuesResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the MeterValues.conf PDU sent by the Central System to the Charge Point in response to a MeterValues.req PDU.
pub struct MeterValuesResponse {}
