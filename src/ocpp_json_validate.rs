//! Validator trait for structs with an associated schema
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
/// Errors associated with validating against a json schema
pub enum JsonValidateError {
    /// Error if validation fails
    ValidationError(Vec<String>),
}

impl fmt::Display for JsonValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ValidationError(e) => write!(f, "Validation Error: {:?}", e),
        }
    }
}

impl Error for JsonValidateError {}

/// Trait for structures that can be validated against a schema
pub trait JsonValidate {
    /// Validate schema against json document
    fn schema_validate(&self) -> Result<(), JsonValidateError>;
}

pub use ocpp_json_validate_attribute::json_validate;
