use crate::core::{self, BootNotificationRequest, BootNotificationResponse};
use chrono::Utc;
use ocpp_json_validate::JsonValidate;

#[test]
fn test_boot_notification_response_validates() {
    let bn_res = BootNotificationResponse {
        current_time: Utc::now(),
        status: core::BootNotificationStatus::Accepted,
        interval: 10,
    };

    assert!(bn_res.validate().is_ok());
}

//ARBITRARY LINE OF CODE HERE TO TEST CRATE PUSH?
#[test]
fn test_boot_notification_request_validates() {
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

    assert!(bn_req.validate().is_ok());
}

#[test]
fn test_boot_notification_request_charge_point_model_string_length_limit() {
    let bn_req = BootNotificationRequest {
        charge_point_vendor: "test1".to_string(),
        charge_point_model: "test2 AND SOME ARBITRARILY LONG STRING HERE TO BREAK THINGS"
            .to_string(),
        charge_point_serial_number: Some("test3".to_string()),
        charge_box_serial_number: Some("test4".to_string()),
        firmware_version: Some("test5".to_string()),
        iccid: Some("test6".to_string()),
        imsi: Some("test7".to_string()),
        meter_type: Some("test8".to_string()),
        meter_serial_number: Some("test9".to_string()),
    };

    assert!(bn_req.validate().is_err());
}
