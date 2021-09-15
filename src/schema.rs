extern crate serde_json;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Authorize{
    pub id_tag: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
    pub struct BootNotification{
    pub    charge_point_vendor:             String,
    pub    charge_point_model:              String,
    pub    charge_point_serial_number:      String,
    pub    charge_box_serial_number:        String,
    pub    firmware_version:                String,
    pub    iccid:                           String,
    pub    imsi:                            String,
    pub    meter_type:                      String,
    pub    meter_serial_number:             String,
}


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct ChangeAvailability{
    pub connector_id: String,
}

/*#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]*/