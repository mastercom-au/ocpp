use crate::schema;

fn main() {
    println!("Compiles!");
    let test = schema::BootNotification {
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

    println!("{:?}", test);
}
