// TODO:
// Issues: Unsupported datatypes i.e. chrono's Utc

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
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;
use validator::{Validate, ValidationErrors};

#[cfg(test)]
use test_strategy::Arbitrary;

// -------------------------- REQUEST --------------------------

#[skip_serializing_none]
#[json_validate("../json_schemas/BootNotification.json")]
#[derive(Serialize, Validate, Deserialize, Debug, Clone, Builder)]
#[builder(build_fn(name = "pre_build"))]
#[serde(rename_all = "camelCase")]
// Strip Optional wrapping when in production to allow setters to directly set values
#[cfg_attr(not(test), builder(setter(strip_option)))]
// Testing only
#[cfg_attr(test, derive(Arbitrary))]

/// Field definition of the BootNotification.req PDU sent by the Charge Point to the Central System.
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

// -------------------------- RESPONSE --------------------------
#[skip_serializing_none]
#[json_validate("../json_schemas/BootNotificationResponse.json")]
#[cfg_attr(test, derive(Arbitrary))]
#[derive(Serialize, Validate, Deserialize, Debug, Clone, Builder)]
#[builder(build_fn(name = "pre_build"))]
#[serde(rename_all = "camelCase")]
// Strip Optional wrapping when in production to allow setters to directly set values
#[cfg_attr(not(test), builder(setter(strip_option)))]
// Testing only
/// Field definition of the BootNotification.conf PDU sent by the Central System to the Charge Point in response to a BootNotification.req PDU.
pub struct BootNotificationResponse {
    /// Identifies whether the charge point has been registered with the central server.
    pub status: BootNotificationStatus,
    /// Required. This contains the current time of the Central System.
    pub current_time: DateTime<Utc>,
    /// When status is accepted, contains the heartbeat inverval in seconds. If status is not accepted, contains a timeout value before the charge point can retry bootnotifacion.
    pub interval: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
#[cfg_attr(test, derive(Arbitrary))]

///Struct Definition
pub enum BootNotificationStatus {
    /// Chargepoint accepted by central system
    Accepted,
    /// Acceptance pending. Central system may send messages to retrieve information or prepare the charge point.
    Pending,
    /// Charge point not accepted, i.e. chargepointID is not known
    Rejected,
}

impl BootNotificationRequestBuilder {
    pub fn build(&self) -> Result<BootNotificationRequest, ValidationErrors> {
        let req = self.pre_build().unwrap();
        match req.validate() {
            Ok(_) => Ok(req),
            Err(e) => return Err(e),
        }
    }
}

impl BootNotificationResponseBuilder {
    pub fn build(&self) -> Result<BootNotificationResponse, ValidationErrors> {
        let req = self.pre_build().unwrap();
        match req.validate() {
            Ok(_) => Ok(req),
            Err(e) => return Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use ocpp_json_validate::JsonValidate;
    use test_strategy::proptest;

    #[proptest]
    fn test_request_builder(proptest_struct: super::BootNotificationRequest) {
        use super::BootNotificationRequestBuilder;
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

        assert_eq!(built_struct.is_ok(), proptest_struct.validate().is_ok());
    }

    #[proptest]
    fn test_response_builder(proptest_struct: super::BootNotificationResponse) {
        use super::BootNotificationResponseBuilder;
        let v = proptest_struct.clone();
        let built_struct = BootNotificationResponseBuilder::default().status(v.status).current_time(v.current_time).interval(v.interval).build();

        assert_eq!(built_struct.is_ok(), proptest_struct.validate().is_ok());
    }
}
