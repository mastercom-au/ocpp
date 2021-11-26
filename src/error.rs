use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidateError {
    //FailToReadSchema(String),
    //FailToCompileSchema(String),
    #[error("Failed to validate struct {0:?}")]
    FailToValidateJson(Vec<String>),
}
