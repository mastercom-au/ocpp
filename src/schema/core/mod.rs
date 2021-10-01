//#[macro_use]
//use chrono::DateTime;
//use chrono::Utc;

//use jsonschema;
//mod serde::{Deserialize, Serialize};
//use serde_json;
 
// #[macro_use]
//use serde::{Serialize, Deserialize};
pub mod authorize;
pub mod boot_notification;
pub mod change_availability;
pub mod change_configuration;
pub mod clear_cache;
pub mod data_transfer;
pub mod get_configuration;
pub mod heartbeat;
pub mod meter_values;
pub mod remote_start_transaction;
pub mod remote_stop_transaction;
pub mod reset;
pub mod start_transaction;
pub mod stop_transaction;
pub mod status_notification;
pub mod unlock_connector;
