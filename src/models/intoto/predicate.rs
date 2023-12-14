//! Custom (de)serialization for various predicate types.
//!
//! This module provides an enum `Predicate` and a custom deserialization function
//! to handle different predicate types, including known types such as `SLSAProvenanceV1`
//! and generic `Other` variants.

use super::provenance::SLSAProvenanceV1Predicate;
use super::scai::SCAIV02Predicate;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// An enum representing different predicate types.
///
/// Known predicate types have their own variants, while unknown types are represented
/// by the `Other` variant, which stores the raw JSON value.
///
/// TODO(mlieberman85): Support (de)serializing the predicates based on the
/// predicateType URL in the statement.
#[derive(Debug, Serialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum Predicate {
    SLSAProvenanceV1(SLSAProvenanceV1Predicate),
    SCAIV02(SCAIV02Predicate),
    Other(Value),
}

// Helper function to deserialize a JSON value into the specified type `T`.
fn deserialize_helper<T: DeserializeOwned>(predicate: &Value) -> Result<T, serde_json::Error> {
    serde_json::from_value::<T>(predicate.clone())
}

/// Deserializes a predicate based on the provided predicate_type.
///
/// If the predicate_type matches a known type, it will deserialize
/// the predicate to the corresponding struct, otherwise, it will
/// deserialize the predicate to the generic `Other` variant.
/// Update the match for any new predicate types.
pub fn deserialize_predicate(
    predicate_type: &str,
    predicate_json: &Value,
) -> Result<Predicate, serde_json::Error> {
    match predicate_type {
        "https://slsa.dev/provenance/v1" => {
            let slsa_provenance = deserialize_helper::<SLSAProvenanceV1Predicate>(predicate_json)?;
            Ok(Predicate::SLSAProvenanceV1(slsa_provenance))
        }
        "https://in-toto.io/attestation/scai/attribute-report" => {
            let scai_v02 = deserialize_helper::<SCAIV02Predicate>(predicate_json)?;
            Ok(Predicate::SCAIV02(scai_v02))
        }
        _ => {
            let other_predicate = deserialize_helper::<Value>(predicate_json)?;
            Ok(Predicate::Other(other_predicate))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_slsa_provenance_v1_predicate() {
        let predicate_type = "https://slsa.dev/provenance/v1";
        let predicate_json = json!({
            "buildDefinition": {
                "buildType": "https://slsa.dev/provenance/v1",
                "externalParameters": {},
                "internalParameters": {},
                "resolvedDependencies": []
            },
            "runDetails": {
                "builder": {
                    "id": "https://example.com/builder"
                },
                "metadata": {
                    "invocationId": "test-invocation-id",
                    "startedOn": "2022-01-01T00:00:00Z"
                }
            }
        });

        let result = deserialize_predicate(predicate_type, &predicate_json);
        assert!(matches!(result, Ok(Predicate::SLSAProvenanceV1(_))));
    }

    #[test]
    fn test_deserialize_other_predicate() {
        let predicate_type = "https://unknown.example.com";
        let predicate_json = json!({
            "key": "value",
            "nested": {
                "a": 1,
                "b": "two"
            }
        });

        let result = deserialize_predicate(predicate_type, &predicate_json);
        assert!(matches!(result, Ok(Predicate::Other(_))));
    }

    #[test]
    fn test_deserialize_invalid_predicate() {
        let predicate_type = "https://slsa.dev/provenance/v1";
        let predicate_json = json!({
            "invalid": "data"
        });

        let result = deserialize_predicate(predicate_type, &predicate_json);
        assert!(result.is_err());
    }
}
