extern crate serde_json;
extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct Authorize{
    idTag: String,
}