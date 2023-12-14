//! In-Toto v1 statement model and associated structures.
//!
//! This module provides the InTotoStatementV1 struct, as well as related structures for
//! subjects, algorithms, and digest sets. It also includes custom (de)serialization
//! code for handling In-Toto v1 statements.

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use url::Url;
use std::fmt::Debug;

use crate::models::intoto::predicate::{deserialize_predicate, Predicate};

/// Represents an In-Toto v1 statement.
#[derive(Debug, Serialize, PartialEq, JsonSchema)]
pub struct InTotoStatementV1<T: Debug + Serialize + PartialEq + JsonSchema = Predicate> {
    #[serde(rename = "_type")]
    #[schemars(with = "Url")]
    pub _type: Url,
    pub subject: Vec<Subject>,
    #[serde(rename = "predicateType")]
    #[schemars(with = "Url")]
    pub predicate_type: Url,
    pub predicate: T,
}

/// Enum for the supported hashing algorithms.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Algorithm {
    // TODO(mlieberman85): Add validation for the length/encoding of the digest string.
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Shake128,
    Shake256,
    Blake2b,
    Blake2s,
    Ripemd160,
    Sm3,
    Gost,
    Sha1,
    Md5,
}

/// Represents a set of digests, mapping algorithms to their respective digest strings.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DigestSet(HashMap<Algorithm, String>);

/// Represents a subject in an In-Toto v1 statement.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Subject {
    pub name: String,
    pub digest: DigestSet,
}

// Custom deserialization for InTotoStatementV1.
impl<'de> Deserialize<'de> for InTotoStatementV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Helper struct to deserialize the JSON before constructing the InTotoStatementV1.
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "_type")]
            _type: Url,
            subject: Vec<Subject>,
            #[serde(rename = "predicateType")]
            predicate_type: Url,
            predicate: Value,
        }

        let helper = Helper::deserialize(deserializer)?;

        // Deserialize the predicate based on the predicate type.
        let predicate = deserialize_predicate(&helper.predicate_type.as_str(), &helper.predicate)
            .map_err(serde::de::Error::custom)?;

        Ok(InTotoStatementV1 {
            _type: helper._type,
            subject: helper.subject,
            predicate_type: helper.predicate_type,
            predicate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_valid_intoto_statement() {
        let json_data = r#"{
            "_type": "https://in-toto.io/Statement/v1",
            "predicateType": "https://random.type/predicate/v1",
            "predicate": {},
            "subject": [
                {
                    "name": "example",
                    "digest": {
                        "sha256": "abcd1234"
                    }
                }
            ]
        }"#;

        let statement: InTotoStatementV1 = serde_json::from_str(json_data).unwrap();
        assert_eq!(statement._type.as_str(), "https://in-toto.io/Statement/v1");
        assert_eq!(
            statement.predicate_type.as_str(),
            "https://random.type/predicate/v1"
        );
        assert_eq!(statement.subject[0].name, "example");
    }

    #[test]
    fn deserialize_intoto_statement_missing_type() {
        let json_data = r#"{
            "predicateType": "https://slsa.dev/provenance/v1",
            "predicate": {},
            "subject": [
                {
                    "name": "example",
                    "digest": {
                        "sha256": "abcd1234"
                    }
                }
            ]
        }"#;

        let result: Result<InTotoStatementV1, _> = serde_json::from_str(json_data);
        assert!(
            result.is_err(),
            "Deserialization should fail due to missing _type field"
        );
    }

    #[test]
    fn deserialize_intoto_statement_invalid_type() {
        let json_data = r#"{
            "_type": "https://example.com/invalid",
            "predicateType": "https://slsa.dev/provenance/v1",
            "predicate": {},
            "subject": [
                {
                    "name": "example",
                    "digest": {
                        "sha256": "abcd1234"
                    }
                }
            ]
        }"#;

        let result: Result<InTotoStatementV1, _> = serde_json::from_str(json_data);
        assert!(
            result.is_err(),
            "Deserialization should fail due to invalid _type field"
        );
    }

    #[test]
    fn deserialize_intoto_statement_missing_subject() {
        let json_data = r#"{
            "_type": "https://in-toto.io/Statement/v1",
            "predicateType": "https://slsa.dev/provenance/v1",
            "predicate": {}
        }"#;

        let result: Result<InTotoStatementV1, _> = serde_json::from_str(json_data);
        assert!(
            result.is_err(),
            "Deserialization should fail due to missing subject field"
        );
    }

    #[test]
    fn deserialize_intoto_statement_missing_predicate_type() {
        let json_data = r#"{
            "_type": "https://in-toto.io/Statement/v1",
            "predicate": {},
            "subject": [
                {
                    "name": "example",
                    "digest": {
                        "sha256": "abcd1234"
                    }
                }
            ]
        }"#;

        let result: Result<InTotoStatementV1, _> = serde_json::from_str(json_data);
        assert!(
            result.is_err(),
            "Deserialization should fail due to missing predicateType field"
        );
    }

    #[test]
    fn deserialize_intoto_statement_missing_predicate() {
        let json_data = r#"{
            "_type": "https://in-toto.io/Statement/v1",
            "predicateType": "https://slsa.dev/provenance/v1",
            "subject": [
                {
                    "name": "example",
                    "digest": {
                        "sha256": "abcd1234"
                    }
                }
            ]
        }"#;

        let result: Result<InTotoStatementV1, _> = serde_json::from_str(json_data);
        assert!(
            result.is_err(),
            "Deserialization should fail due to missing predicate field"
        );
    }

    #[test]
    fn deserialize_intoto_statement_invalid_subject() {
        let json_data = r#"{
            "_type": "https://in-toto.io/Statement/v1",
            "predicateType": "https://slsa.dev/provenance/v1",
            "predicate": {},
            "subject": [
                {
                    "name": "example",
                    "digest": {
                        "sha256": "invalid_digest"
                    }
                }
            ]
        }"#;

        let result: Result<InTotoStatementV1, _> = serde_json::from_str(json_data);
        assert!(
            result.is_err(),
            "Deserialization should fail due to invalid digest in the subject"
        );
    }
}
