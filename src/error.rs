use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ValidateError {
    //FailToReadSchema(String),
    //FailToCompileSchema(String),
    FailToValidateJson(Vec<String>),
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            //Self::FailToReadSchema(inner) => write!(f, "Failed to read schema: {}", inner),
            //Self::FailToCompileSchema(inner) => {write!(f, "Could not compile schema to json: {}", inner)}
            Self::FailToValidateJson(inner) => {
                write!(f, "json::FailToValidateStruct {:?}", inner)
            }
        }
    }
}

impl Error for ValidateError {}
