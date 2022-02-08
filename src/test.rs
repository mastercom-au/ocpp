use crate::point_init::boot_notification::*;
//use crate::server_init::*;
use chrono::Utc;
use ocpp_json_validate::JsonValidate;

#[test]
fn test_boot_notification_response_validates() {
    let bn_res = BootNotificationResponse {
        current_time: Utc::now(),
        status: BootNotificationStatus::Accepted,
        interval: 10,
    };

    assert!(bn_res.validate().is_ok());
}

fn implements_display<T: std::fmt::Display>() {}
#[test]
fn test_enum_display() { implements_display::<crate::common_types::SimpleStatus>(); }

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
        meter_type: None,
        meter_serial_number: None,
    };
    assert!(bn_req.validate().is_ok());
}

#[test]
fn test_boot_notification_request_charge_point_model_string_length_limit() {
    let bn_req = BootNotificationRequest {
        charge_point_vendor: "test1".to_string(),
        charge_point_model: "test2 AND SOME ARBITRARILY LONG STRING HERE TO BREAK THINGS".to_string(),
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

#[test]
fn test_deserialize_json_call() {
    let json = "[2,\"63:2\",\"StatusNotification\",{\"connectorId\":0,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"2022-01-24T04:30:50.621Z\"}]";
    let value: crate::common_types::JsonCall = serde_json::from_str(json).unwrap();

    assert_eq!(value.0, 2);
}
