mod diagnostic_status_notification;
pub use diagnostic_status_notification::{DiagnosticStatusNotificationRequest, DiagnosticStatusNotificationResponse};

mod firmware_status_notification;
pub use firmware_status_notification::{FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse};


mod get_diagnostics;
pub use get_diagnostics::{GetDiagnosticsRequest, GetDiagnosticsResponse };


mod update_firmware;
pub use update_firmware::{UpdateFirmwareRequest, UpdateFirmwareResponse};


