use crate::core;
use jsonschema::JSONSchema;
//use serde_json::json;

#[test]
fn test() {
    let boot_notification_test = core::BootNotificationRequest {
        charge_point_vendor: "test1".to_string(),
        charge_point_model: "THIS_FIELD_IS_WAY_LONGER_THAN_SHOULD_BE_REASONABLY_ALLOWED_ACCORDING_TO_THE_SCHEMA".to_string(),
        charge_point_serial_number: Some("test3".to_string()),
        charge_box_serial_number: Some("test4".to_string()),
        firmware_version: Some("test5".to_string()),
        iccid: Some("test6".to_string()),
        imsi: Some("test7".to_string()),
        meter_type: Some("test8".to_string()),
        meter_serial_number: Some("test9".to_string()),
    };

    let string_schema = String::from(include_str!("json_schemas/core/BootNotification.json"));

    println!("\n\nString: \n\n {}", string_schema);

    let json_schema = serde_json::from_str(&string_schema).unwrap();

    let value = serde_json::to_value(boot_notification_test).unwrap();
    if let Ok(compiled_schema) = JSONSchema::compile(&json_schema){
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

}   



/*fn test_schema(schema_path: String) -> Result<{

}*/