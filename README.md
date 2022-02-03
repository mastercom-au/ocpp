# Description
Crate for serializing, deserializing and validating data packets from an OCPP point.

# Overview
Ocpp is a rust crate that defines data structures and methods useful for communicating with an OCPP point server. The definitions are separated into four categories which represent various common groups of functionalities as provided by the OCPP definitions. These can be imported as separate modules:
##### 1. Core
Basic Charge Point functionality comparable with OCPP 1.5 without support for firmware updates, local
authorization list management and reservations. functionality
##### 2. Authentication List Management:
Features to manage the local authorization list in Charge Points.
##### 3. Firmware Management
Support for firmware update management and diagnostic log file download.
##### 4. Smart Charging
Support for basic Smart Charging, for instance using control pilot.

## Functionality

#### Importing Types
Types can be imported either as a group organised by their initiation point (ChargePoint or Central Server) i.e.:

    use ocpp::server_init::*
    use ocpp::point_init::*

or individually: 

    use ocpp::server_init::reset::*;
    use ocpp::point_init::boot_notification::*;

using * to import will export both the request and response messages, as well as allowing easy access to the individual components of the struct.

#### Validate
	fn validate(&self) -> Result<(), ValidateError> {...}
Checks for validation against the relevant schema for a request or response struct defined within the crate. Returns ValidateError if invalid, which implements display to parse a Vec of Strings detailing any errors with the definition.

# Example/Usage
	use crate::core::{self, BootNotificationRequest, BootNotificationResponse};
	use crate::validate::JsonValidate;
	use chrono::Utc;

	let bn_res = BootNotificationResponse {
        current_time: Utc::now(),
        status: core::BootNotificationStatus::Accepted,
        interval: 10,
    };

    let bn_req = BootNotificationRequest {
        charge_point_vendor: "BigElectric".to_string(),
        charge_point_model: "SOME ARBITRARILY LONG STRING HERE TO DEMONSTRATE THE SCHEMA WILL THROW AN ERROR FOR THIS DEFINITION"
            .to_string(),
        charge_point_serial_number: Some("test3".to_string()),
        charge_box_serial_number: Some("test4".to_string()),
        firmware_version: Some("test5".to_string()),
        iccid: Some("test6".to_string()),
        imsi: Some("test7".to_string()),
        meter_type: Some("test8".to_string()),
        meter_serial_number: Some("test9".to_string()),
    };

    assert!(bn_res.validate().is_ok());
    assert!(bn_req.validate().is_err());



