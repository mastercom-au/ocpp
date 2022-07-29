//! Validator trait for structs with an associated schema
use std::error::Error;
use std::fmt;

pub use proc_macros::json_validate;
pub use proc_macros::BuilderValidator;

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


#[macro_export]
/// Expands to the builder for a particular OCPP structure
macro_rules! generate_validation_comparison_tests {
    ($i:expr) => {
        paste::paste!{
        #[cfg(test)]
        mod test {
            use super::*;
            use test_strategy::proptest;
        
            #[proptest]
            fn request_struct_validation_matches_schema_validation(fuzzed_struct: [<$i Request>]) {
                println!("{:?}", fuzzed_struct);
                assert!([<$i Request>]::compare_validation_methods(fuzzed_struct));
            }
            #[proptest]
            fn response_struct_validation_matches_schema_validation(fuzzed_struct: [<$i Response>]) {
                assert!([<$i Response>]::compare_validation_methods(fuzzed_struct));
            }
        }
    }
    };
}