pub trait JsonValidate {
    fn validate(&self);
}

pub mod boot_notification;
pub use boot_notification::*;

mod authorize;
pub use authorize::{AuthorizeRequest, AuthorizeResponse};

mod change_availability;
pub use change_availability::{ChangeAvailabilityRequest, ChangeAvailabilityResponse};

mod change_configuration;
pub use change_configuration::{ChangeConfigurationRequest, ChangeConfigurationResponse};

mod clear_cache;
pub use clear_cache::{ClearCacheRequest, ClearCacheResponse};

mod data_transfer;
pub use data_transfer::{DataTransferRequest, DataTransferResponse};

mod get_configuration;
pub use get_configuration::{GetConfigurationRequest, GetConfigurationResponse};

mod heartbeat;
pub use heartbeat::{HeartBeatResponse, HeartbeatRequest};

mod meter_values;
pub use meter_values::{MeterValuesRequest, MeterValuesResponse};

mod remote_start_transaction;
pub use remote_start_transaction::{RemoteStartTransactionRequest, RemoteStartTransactionResponse};

mod remote_stop_transaction;
pub use remote_stop_transaction::{RemoteStopTransactionRequest, RemoteStopTransactionResponse};

mod reset;
pub use reset::{ResetRequest, ResetResponse};

mod start_transaction;
pub use start_transaction::{StartTransactionRequest, StartTransactionResponse};

mod status_notification;
pub use status_notification::{StatusNotificationRequest, StatusNotificationResponse};

mod stop_transaction;
pub use stop_transaction::{StopTransactionRequest, StopTransactionResponse};

mod unlock_connector;
pub use unlock_connector::{UnlockConnectorRequest, UnlockConnectorResponse};
