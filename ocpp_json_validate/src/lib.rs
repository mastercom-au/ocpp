use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum JsonValidateError
{
    ValidationError(Vec<String>),
}

impl fmt::Display for JsonValidateError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            Self::ValidationError(e) => write!(f, "Validation Error: {:?}", e),
        }
    }
}

impl Error for JsonValidateError {}



pub trait JsonValidate
{
    fn validate(&self) -> Result<(), JsonValidateError>;
}



pub use ocpp_json_validate_attribute::json_validate;
