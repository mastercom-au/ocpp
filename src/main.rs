use mp70_interface::schema as json;

fn main (){
    println!("Compiles!");
    let test = json::BootNotification {charge_point_vendor:"test1".to_string(),
                                            charge_point_model:"test2".to_string(),
                                            charge_point_serial_number:"test3".to_string(),
                                            charge_box_serial_number:"test4".to_string(),
                                            firmware_version:"test5".to_string(),
                                            iccid:"test6".to_string(),
                                            imsi:"test7".to_string(),
                                            meter_type:"test8".to_string(),
                                            meter_serial_number:"test9".to_string(),
                                            };

    println!("{:?}", test);
}