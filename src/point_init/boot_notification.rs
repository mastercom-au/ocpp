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
use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

// -------------------------- REQUEST --------------------------
#[json_validate("../json_schemas/BootNotification.json")]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Field definition of the BootNotification.req PDU sent by the Charge Point to the Central System.
pub struct BootNotificationRequest {
    /// Optional. This contains a value that identifies the serial number of the Charge Box inside the Charge Point.
    /// Deprecated, will be removed in future versio
    pub charge_point_vendor: String,
    /// Required. This contains a value that identifies the model of the ChargePoint.
    pub charge_point_model: String,
    /// Optional. This contains a value that identifies the serial number of the Charge Point.
    pub charge_point_serial_number: Option<String>,
    /// Identifies the serial number of the Charge Box inside the Charge Point. Deprecated, will be removed in future version.
    pub charge_box_serial_number: Option<String>,
    /// Identifies the firmware version on the charge .
    pub firmware_version: Option<String>,
    /// Identifies the ICCID of the modem's SIM card.
    pub iccid: Option<String>,
    /// Identifies the IMSI of the modem's SIM card.
    pub imsi: Option<String>,
    /// Identifies the type of the main electrical meter of the charge point.
    pub meter_type: Option<String>,
    /// Identifies the serial numbver of the main electrical meter of the charge point.
    pub meter_serial_number: Option<String>,
}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/BootNotificationResponse.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
///Struct Definition
pub enum BootNotificationStatus {
    /// Chargepoint accepted by central system
    Accepted,
    /// Acceptance pending. Central system may send messages to retrieve information or prepare the charge point.
    Pending,
    /// Charge point not accepted, i.e. chargepointID is not known
    Rejected,
}
