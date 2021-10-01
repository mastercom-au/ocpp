use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    r#type: ResetType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ResetType {
    Hard,
    Soft,
}