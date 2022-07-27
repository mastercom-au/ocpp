//! Initialization message detailing general information about the charge point (e.g version, vendor etc.).
//!
//!  # Behaviour
//! After start-up, a Charge Point SHALL send a request to the Central System with information about its
//! configuration (e.g. version, vendor, etc.). The Central System SHALL respond to indicate whether it will accept the
//! Charge Point.
//! The Charge Point SHALL send a BootNotification.req PDU each time it boots or reboots. Between the physical
//! power-on/reboot and the successful completion of a BootNotification, where Central System returns Accepted or
//! Pending, the Charge Point SHALL NOT send any other request to the Central System. This includes cached
//! messages that are still present in the Charge Point from before.
//!
//!  # Response
//! When the Central System responds with a BootNotification.conf with a status Accepted, the Charge Point will
//! adjust the heartbeat interval in accordance with the interval from the response PDU and it is RECOMMENDED to
//! synchronize its internal clock with the supplied Central System’s current time. If the Central System returns
//! something other than Accepted, the value of the interval field indicates the minimum wait time before sending a
//! next BootNotification request. If that interval value is zero, the Charge Point chooses a waiting interval on its
//! own, in a way that avoids flooding the Central System with requests. A Charge Point SHOULD NOT send a
//! BootNotification.req earlier, unless requested to do so with a TriggerMessage.req.
//!
//! If the Central System returns the status Rejected, the Charge Point SHALL NOT send any OCPP message to the
//! Central System until the aforementioned retry interval has expired. During this interval the Charge Point may no
//! longer be reachable from the Central System. It MAY for instance close its communication channel or shut down
//! its communication hardware. Also the Central System MAY close the communication channel, for instance to
//! free up system resources. While Rejected, the Charge Point SHALL NOT respond to any Central System initiated
//! message. the Central System SHOULD NOT initiate any.
//!
//! The Central System MAY also return a Pending registration status to indicate that it wants to retrieve or set
//! certain information on the Charge Point before the Central System will accept the Charge Point. If the Central
//! System returns the Pending status, the communication channel SHOULD NOT be closed by either the Charge
//! Point or the Central System. The Central System MAY send request messages to retrieve information from the
//! Charge Point or change its configuration. The Charge Point SHOULD respond to these messages. The Charge
//! Point SHALL NOT send request messages to the Central System unless it has been instructed by the Central
//! System to do so with a TriggerMessage.req request.
//!
//! While in pending state, the following Central System initiated messages are not allowed:
//! RemoteStartTransaction.req and RemoteStopTransaction.req
use crate::{error::OcppError, generate_builders, UtcTime};
use derive_builder::Builder;
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;
use validator::Validate;

#[cfg(test)]
use test_strategy::Arbitrary;

// -------------------------- REQUEST --------------------------

/// Field definition of the BootNotification.req PDU sent by the Charge Point to the Central System.
#[skip_serializing_none]
#[json_validate("../json_schemas/BootNotification.json")]
#[derive(Serialize, Validate, Deserialize, Debug, Clone, Builder)]
#[builder(build_fn(name = "pre_build", error = "OcppError"))]
#[serde(rename_all = "camelCase")]
// Strip Optional wrapping when in production to allow setters to directly set values
#[cfg_attr(not(test), builder(setter(strip_option)))]
// Testing only
#[cfg_attr(test, derive(Arbitrary))]
pub struct BootNotificationRequest {
    /// Optional. This contains a value that identifies the serial number of the Charge Box inside the Charge Point.
    /// Deprecated, will be removed in future versio
    #[validate(length(max = 20))]
    pub charge_point_vendor: String,
    /// Required. This contains a value that identifies the model of the ChargePoint.
    #[validate(length(max = 20))]
    pub charge_point_model: String,
    /// Optional. This contains a value that identifies the serial number of the Charge Point.
    #[validate(length(max = 25))]
    #[builder(default)]
    pub charge_point_serial_number: Option<String>,
    /// Identifies the serial number of the Charge Box inside the Charge Point. Deprecated, will be removed in future version.
    #[validate(length(max = 25))]
    #[builder(default)]
    pub charge_box_serial_number: Option<String>,
    /// Identifies the firmware version on the charge .
    #[validate(length(max = 50))]
    #[builder(default)]
    pub firmware_version: Option<String>,
    /// Identifies the ICCID of the modem's SIM card.
    #[validate(length(max = 20))]
    #[builder(default)]
    pub iccid: Option<String>,
    /// Identifies the IMSI of the modem's SIM card.
    #[validate(length(max = 20))]
    #[builder(default)]
    pub imsi: Option<String>,
    /// Identifies the type of the main electrical meter of the charge point.
    #[validate(length(max = 25))]
    #[builder(default)]
    pub meter_type: Option<String>,
    /// Identifies the serial numbver of the main electrical meter of the charge point.
    #[validate(length(max = 25))]
    #[builder(default)]
    pub meter_serial_number: Option<String>,
}

// impl BootNotificationRequestBuilder {
//     pub fn build(&self) -> Result<BootNotificationRequest, OcppError> {
//         let req = self.pre_build()?;
//         return req.validate().map(|_| req).map_err(|e| e.into());
//     }
// }
// -------------------------- RESPONSE --------------------------
/// Field definition of the BootNotification.conf PDU sent by the Central System to the Charge Point in response to a BootNotification.req PDU.
#[skip_serializing_none]
#[json_validate("../json_schemas/BootNotificationResponse.json")]
#[derive(Serialize, Validate, Deserialize, Debug, Clone, Builder)]
#[builder(build_fn(name = "pre_build", error = "OcppError"))]
#[serde(rename_all = "camelCase")]
// Strip Optional wrapping when in production to allow setters to directly set values
#[cfg_attr(not(test), builder(setter(strip_option)))]
// Testing only
#[cfg_attr(test, derive(Arbitrary))]
pub struct BootNotificationResponse {
    /// Identifies whether the charge point has been registered with the central server.
    pub status: BootNotificationStatus,
    /// Required. This contains the current time of the Central System.
    pub current_time: UtcTime,
    /// When status is accepted, contains the heartbeat inverval in seconds. If status is not accepted, contains a timeout value before the charge point can retry bootnotifacion.
    pub interval: u32,
}

///Struct Definition
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum BootNotificationStatus {
    /// Chargepoint accepted by central system
    Accepted,
    /// Acceptance pending. Central system may send messages to retrieve information or prepare the charge point.
    Pending,
    /// Charge point not accepted, i.e. chargepointID is not known
    Rejected,
}

generate_builders!(BootNotification);

#[cfg(test)]
mod test {
    use super::*;
    use ocpp_json_validate::JsonValidate;
    use test_strategy::proptest;

    /// Test validation via builder against validation via schema
    #[proptest]
    fn compare_request_builder_validation_with_schema_validation(proptest_struct: super::BootNotificationRequest) {
        let v = proptest_struct.clone();
        let built_struct = BootNotificationRequestBuilder::default()
            .charge_point_vendor(v.charge_point_vendor)
            .charge_point_model(v.charge_point_model.clone())
            .charge_point_serial_number(v.charge_point_serial_number)
            .charge_box_serial_number(v.charge_box_serial_number)
            .firmware_version(v.firmware_version)
            .iccid(v.iccid)
            .imsi(v.imsi)
            .meter_type(v.meter_type)
            .meter_serial_number(v.meter_serial_number)
            .build();

        let builder_validated_ok = built_struct.is_ok();
        let schema_validated_ok = proptest_struct.schema_validate().is_ok();
        assert_eq!(builder_validated_ok, schema_validated_ok);
    }

    #[proptest]
    fn compare_response_builder_validation_with_schema_validation(proptest_struct: super::BootNotificationResponse) {
        let v = proptest_struct.clone();
        let built_struct = BootNotificationResponseBuilder::default().status(v.status).current_time(v.current_time).interval(v.interval).build();

        let builder_validated_ok = built_struct.is_ok();
        let schema_validated_ok = proptest_struct.schema_validate().is_ok();
        assert_eq!(builder_validated_ok, schema_validated_ok);
    }
}
