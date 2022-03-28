use crate::{point_init::boot_notification::*, ChargingSchedulePeriod};
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
fn test_deserialize_json_call() -> Result<(), Box<dyn std::error::Error>> {
    let json = "[2,\"63:2\",\"StatusNotification\",{\"connectorId\":0,\"errorCode\":\"NoError\",\"status\":\"Available\",\"timestamp\":\"2022-01-24T04:30:50.621Z\"}]";
    let value: crate::OCPPMessage = serde_json::from_str(json)?;

    assert!(matches!(value, crate::OCPPMessage::Call(..)));

    if let crate::OCPPMessage::Call(call) = value {
        assert!(matches!(call.payload, crate::OCPPCallPayload::StatusNotification(..)));
    }

    Ok(())
}

#[test]
fn test_deserialize_json_call_result() -> Result<(), Box<dyn std::error::Error>> {
    let json = "[3,\"63:2\",{}]";
    let value: crate::OCPPMessage = serde_json::from_str(json)?;

    assert!(matches!(value, crate::OCPPMessage::CallResultUnknown(..)));

    if let crate::OCPPMessage::CallResultUnknown(unknown) = value {
        let result = crate::OCPPCallResult::from_unknown(&crate::OCPPCallAction::StatusNotification, unknown)?;

        assert!(matches!(result.payload, crate::OCPPCallResultPayload::StatusNotification(..)));
    }

    Ok(())
}

#[test]
fn test_serialize_get_configuration_call() -> Result<(), Box<dyn std::error::Error>> {
    let req = crate::GetConfigurationRequest { key: None };
    req.validate()?;

    let message = crate::OCPPMessage::Call((String::from("64:1"), crate::OCPPCallPayload::GetConfiguration(req)).into());
    let json = serde_json::to_string(&message)?;

    let expected = "[2,\"64:1\",\"GetConfiguration\",{}]";

    assert_eq!(json, expected);

    Ok(())
}

#[test]
fn test_charge_point_builder() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{ChargingProfile, ChargingRateUnit, ChargingSchedule};

    let builder = ChargingProfile::builder();
    let mut charging_schedule_period: Vec<ChargingSchedulePeriod> = Vec::new();

    charging_schedule_period.push(ChargingSchedulePeriod { start_period: 5, limit: 0.5, number_phases: None });
    let charging_schedule = ChargingSchedule {
        duration: None,
        start_schedule: None,
        charging_rate_unit: ChargingRateUnit::A,
        charging_schedule_period,
        min_charging_rate: None,
    };

    let builder = builder.schedule(charging_schedule).id(999).level(5);
    let profile = builder.build();

    assert_eq!(profile.charging_profile_id == 5).into()
}
