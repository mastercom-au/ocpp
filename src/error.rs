//! Error handling

use derive_builder::UninitializedFieldError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
/// Errors related to building an OCPP object
pub enum OcppError {
    #[error("Field missing from builder")]
    /// Error resultant from a missing field when building an OCPP object
    BuilderError(#[from] UninitializedFieldError),
    #[error("Struct is invalid")]
    /// Error resultant from bad field when building an OCPP object
    OcppValidationError(#[from] ValidationErrors),
}
