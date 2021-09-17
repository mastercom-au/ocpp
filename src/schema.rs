extern crate serde_json;
extern crate serde;
extern crate jsonschema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Authorize{
    pub id_tag:                         String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotification{
    pub charge_point_vendor: String,
    pub charge_point_model: String,
    pub charge_point_serial_number: String,
    pub charge_box_serial_number: String,
    pub firmware_version: String,
    pub iccid: String,
    pub imsi: String,
    pub meter_type: String,
    pub meter_serial_number: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailability{
    pub connector_id: u32,
    pub type: enum type{"Inoperative","operative"}
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfiguration{
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCache{
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransfer{
    pub vendor_id: String,
    pub message_id: String,
    pub data: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConfiguration{
    pub key: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat{
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValues{
    //RETURN TO ME 
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransaction{
    //RETURN TO ME 
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransaction{
    //RETURN TO ME 
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reset{
    //RETURN TO ME 
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransaction{
    pub connector_id: 
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransaction{

}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotification{

}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnector{

}


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]*/