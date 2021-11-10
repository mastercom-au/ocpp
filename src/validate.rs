use crate::error::ValidateError;
use serde::Serialize;
use serde_json::json;

pub trait JsonValidate {
    fn generic_validate(
        &self,
        schema_validator: &jsonschema::JSONSchema,
    ) -> Result<(), ValidateError>
    where
        Self: Serialize,
    {
        let mut output = Vec::new();
        if let Err(errors) = schema_validator.validate(&json!(&self)) {
            for error in errors {
                //ValidationError references the json document that it was generated from.
                //Stripping it down to a string stops errors generated when it drops from scope
                output.push(error.to_string());
            }
            return Err(ValidateError::FailToValidateJson(output));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), ValidateError> {
        return Ok(());
    }
}
