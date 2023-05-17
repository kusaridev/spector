//! Validator trait and implementations for validating JSON values.
//!
//! This currently supports JSONSchema validation.
//! This additional validation helps ensure all errors during validation are returned to the user.
//! Serde will short-circuit on the first error it encounters. Thi means that if there are multiple
//! the user will have to correct an error in their doc and repeat until Spector reports no more errors.

use anyhow::{anyhow, Result};
use jsonschema::JSONSchema;
use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};

/// A trait for implementing validation logic on JSON values.
pub trait Validator {
    type Output;

    /// Validates the given JSON value and assuming no errors returns the deserialized output.
    fn validate(&self, value: &Value) -> Result<Self::Output>;
}

/// A JSON Schema-based validator for JSON values.
///
/// The `JSONSchemaValidator` struct uses a JSON Schema to validate a JSON value and
/// then deserializes if it is valid into the specified output type.
pub struct JSONSchemaValidator<T: DeserializeOwned> {
    schema: Value,

    // TODO(mlieberman85): this using phantomdata seems like an easy way to tell it return a deserialized values
    // but I should probably look into if I can make this simpler.
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> JSONSchemaValidator<T> {
    /// Creates a new JSONSchemaValidator with the given JSON Schema.
    pub fn new(schema: &Value) -> Self {
        Self {
            schema: schema.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: DeserializeOwned> Validator for JSONSchemaValidator<T> {
    type Output = T;

    fn validate(&self, value: &Value) -> Result<Self::Output> {
        let schema = JSONSchema::compile(&self.schema)
            .map_err(|e| anyhow!("Failed to compile schema: {}", e))?;

        let validate = schema.validate(value);

        match validate {
            Ok(_) => {
                let deserialized_value = from_value(value.clone())
                    .map_err(|e| anyhow!("Failed to deserialize value: {}", e))?;
                Ok(deserialized_value)
            }
            Err(e) => {
                let error_messages = e
                    .map(|e| {
                        format!(
                            "{}\npath: {}",
                            serde_json::to_string_pretty(&e.instance).unwrap_or(e.to_string()),
                            e.instance_path
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                Err(anyhow!("Failed to validate JSON value: {}", error_messages))
            }
        }
    }
}

pub struct GenericValidator<T: DeserializeOwned> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> Validator for GenericValidator<T> {
    type Output = T;

    fn validate(&self, value: &Value) -> Result<Self::Output> {
        let deserialized_value = from_value(value.clone())
            .map_err(|e| anyhow!("Failed to deserialize value into type: {}", e))?;
        Ok(deserialized_value)
    }
}

impl<T: DeserializeOwned> GenericValidator<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::*;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    fn person_schema() -> Value {
        json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer" }
            },
            "required": ["name", "age"]
        })
    }

    #[test]
    fn test_jsonschema_valid_person() {
        let schema = person_schema();
        let validator = JSONSchemaValidator::<Person>::new(&schema);

        let valid_value = json!({
            "name": "John Doe",
            "age": 30
        });

        let person = validator.validate(&valid_value).unwrap();
        assert_eq!(
            person,
            Person {
                name: "John Doe".into(),
                age: 30
            }
        );
    }

    #[test]
    fn test_jsonschema_invalid_person() {
        let schema = person_schema();
        let validator = JSONSchemaValidator::<Person>::new(&schema);

        let invalid_value = json!({
            "name": 123,
            "age": "thirty"
        });

        assert!(validator.validate(&invalid_value).is_err());
    }

    #[test]
    fn test_generic_person_validation() {
        let validator = GenericValidator::<Person>::new();
        let json_value = json!({
            "name": "John Doe",
            "age": 30
        });
        let expected = Person {
            name: String::from("John Doe"),
            age: 30
        };
        let result = validator.validate(&json_value).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generic_json_value_validation() {
        let validator = GenericValidator::<Value>::new();
        let json_value = json!({
            "key": "value",
            "number": 123
        });
        let expected = json_value.clone();
        let result = validator.validate(&json_value).unwrap();
        assert_eq!(result, expected);
    }
}
