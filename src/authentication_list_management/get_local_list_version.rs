use ocpp_json_validate::json_validate;
use serde::{Deserialize, Serialize};

// -------------------------- REQUEST --------------------------
#[json_validate("../json_schemas/Requests/AuthenticationListManagement/GetLocalListVersion.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {}

// -------------------------- RESPONSE --------------------------
#[json_validate("../json_schemas/Responses/AuthenticationListManagement/GetLocalListVersion.json")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    pub list_version: u32,
}
