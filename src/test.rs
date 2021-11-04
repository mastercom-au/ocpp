use std::fs::read_to_string;

use crate::core::{self, BootNotificationRequest, BootNotificationResponse, JsonValidate};
use chrono::{Date, DateTime, Utc};
use jsonschema::JSONSchema;
use serde_json;

pub const REQUESTPATH: &str = "json_schemas/Requests/Core/BootNotification.json";

#[test]
fn test() {
    let bn_res = BootNotificationResponse {
        current_time: Utc::now(),
        status: core::BootNotificationStatus::Accepted,
        interval: 10,
    };

    let bn_req = BootNotificationRequest {
        charge_point_vendor: "test1".to_string(),
        charge_point_model: "test2".to_string(),
        charge_point_serial_number: Some("test3".to_string()),
        charge_box_serial_number: Some("test4".to_string()),
        firmware_version: Some("test5".to_string()),
        iccid: Some("test6".to_string()),
        imsi: Some("test7".to_string()),
        meter_type: Some("test8".to_string()),
        meter_serial_number: Some("test9".to_string()),
    };

    bn_req.validate();
}

/*fn test_schema(schema_path: String) -> Result<{

}*/

/*
let value = serde_json::to_value(boot_notification_test).unwrap();
if let Ok(compiled_schema) = JSONSchema::compile(&json_schema) {
    let result = compiled_schema.validate(&value);
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error)
        }
        panic!("Validation Error")
    }
} else {
    panic!("Compile of JSON failed")
}
*/
