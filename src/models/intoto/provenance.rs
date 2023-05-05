//! SLSA provenance predicate model and associated structures.
//!
//! This module provides structs for the SLSAProvenanceV1Predicate and its related structures.
//! It also includes the necessary (de)serialization code for handling SLSA provenance predicates.

use crate::models::helpers::{b64_option_serde, url_serde};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/// A structure representing the SLSA Provenance v1 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct SLSAProvenanceV1Predicate {
    #[serde(rename = "buildDefinition")]
    pub build_definition: BuildDefinition,
    #[serde(rename = "runDetails")]
    pub run_details: RunDetails,
}

/// A structure representing the build definition of the SLSA Provenance v1 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct BuildDefinition {
    #[serde(rename = "buildType", with = "url_serde")]
    #[schemars(with = "Url")]
    pub build_type: Url,
    #[serde(rename = "externalParameters")]
    pub external_parameters: serde_json::Value,
    #[serde(rename = "internalParameters")]
    pub internal_parameters: serde_json::Value,
    #[serde(rename = "resolvedDependencies")]
    pub resolved_dependencies: Vec<ResourceDescriptor>,
}

/// A structure representing the run details of the SLSA Provenance v1 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct RunDetails {
    pub builder: Builder,
    pub metadata: Metadata,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byproducts: Option<Vec<ResourceDescriptor>>,
}

/// A structure representing the builder information of the SLSA Provenance v1 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Builder {
    #[serde(with = "url_serde")]
    #[schemars(with = "Url")]
    pub id: Url,
    #[serde(
        rename = "builderDependencies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub builder_dependencies: Option<Vec<ResourceDescriptor>>,
    pub version: Option<String>,
}

/// A structure representing the metadata of the SLSA Provenance v1 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Metadata {
    #[serde(rename = "invocationId")]
    pub invocation_id: String,
    #[serde(rename = "startedOn")]
    pub started_on: DateTime<Utc>,
    #[serde(rename = "finishedOn")]
    pub finished_on: Option<DateTime<Utc>>,
}

/// A structure representing a resource descriptor in the SLSA Provenance v1 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ResourceDescriptor {
    #[serde(with = "url_serde")]
    #[schemars(with = "Url")]
    pub uri: Url,
    pub digest: Option<HashMap<String, String>>,
    pub name: Option<String>,
    #[serde(
        rename = "downloadLocation",
        with = "url_serde",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "Url")]
    pub download_location: Option<Url>,
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,
    // TODO(mlieberman85): Fix below. Serde was erroring without the default attribute.
    // I think we can probably use a crate with base64 decoding already built in.
    #[serde(
        with = "b64_option_serde",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    // TODO(mlieberman85): Use a base64 type when this issue is resolved:
    // https://github.com/GREsau/schemars/issues/160
    #[schemars(with = "String")]
    pub content: Option<Vec<u8>>,
    pub annotations: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use serde_json::json;

    fn get_test_slsa_provenance() -> SLSAProvenanceV1Predicate {
        SLSAProvenanceV1Predicate {
            build_definition: BuildDefinition {
                build_type: Url::parse("https://example.com/buildType/v1").unwrap(),
                external_parameters: json!({"key": "value"}),
                internal_parameters: json!({"key": "value"}),
                resolved_dependencies: vec![ResourceDescriptor {
                    uri: Url::parse("https://example.com/dependency1").unwrap(),
                    digest: Some(hashmap! {"algorithm1".to_string() => "digest1".to_string()}),
                    name: Some("dependency1".to_string()),
                    download_location: Some(Url::parse("https://example.com/download1").unwrap()),
                    media_type: Some("media/type1".to_string()),
                    content: Some(b"content1".to_vec()),
                    annotations: Some(json!({"key": "value"})),
                }],
            },
            run_details: RunDetails {
                builder: Builder {
                    id: Url::parse("https://example.com/builder/v1").unwrap(),
                    builder_dependencies: Some(vec![ResourceDescriptor {
                        uri: Url::parse("https://example.com/builder/dependency1").unwrap(),
                        digest: Some(hashmap! {"algorithm1".to_string() => "digest1".to_string()}),
                        name: Some("builder_dependency1".to_string()),
                        download_location: Some(
                            Url::parse("https://example.com/builder/download1").unwrap(),
                        ),
                        media_type: Some("media/type1".to_string()),
                        content: Some(b"content1".to_vec()),
                        annotations: Some(json!({"key": "value"})),
                    }]),
                    version: Some("1.0.0".to_string()),
                },
                metadata: Metadata {
                    invocation_id: "invocation1".to_string(),
                    started_on: DateTime::parse_from_rfc3339("2023-01-01T12:34:56Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    finished_on: Some(
                        DateTime::parse_from_rfc3339("2023-01-01T13:34:56Z")
                            .unwrap()
                            .with_timezone(&Utc),
                    ),
                },
                byproducts: Some(vec![ResourceDescriptor {
                    uri: Url::parse("https://example.com/byproduct1").unwrap(),
                    digest: Some(hashmap! {"algorithm1".to_string() => "digest1".to_string()}),
                    name: Some("byproduct1".to_string()),
                    download_location: Some(
                        Url::parse("https://example.com/byproduct/download1").unwrap(),
                    ),
                    media_type: Some("media/type1".to_string()),
                    content: Some(b"content1".to_vec()),
                    annotations: Some(json!({"key": "value"})),
                }]),
            },
        }
    }

    fn get_test_slsa_provenance_json() -> serde_json::Value {
        json!({
                "buildDefinition": {
                    "buildType": "https://example.com/buildType/v1",
                    "externalParameters": {
                        "key": "value"
                    },
                    "internalParameters": {
                        "key": "value"
                    },
                    "resolvedDependencies": [
                        {
                            "uri": "https://example.com/dependency1",
                            "digest": {
                                "algorithm1": "digest1"
                            },
                            "name": "dependency1",
                            "downloadLocation": "https://example.com/download1",
                            "mediaType": "media/type1",
                            "content": "Y29udGVudDE=",
                        "annotations": {
                            "key": "value"
                        },
                    },
                ],
            },
            "runDetails": {
                "builder": {
                    "id": "https://example.com/builder/v1",
                    "builderDependencies": [
                        {
                            "uri": "https://example.com/builder/dependency1",
                            "digest": {
                                "algorithm1": "digest1"
                            },
                            "name": "builder_dependency1",
                            "downloadLocation": "https://example.com/builder/download1",
                            "mediaType": "media/type1",
                            "content": "Y29udGVudDE=",
                            "annotations": {
                                "key": "value"
                            },
                        },
                    ],
                    "version": "1.0.0",
                },
                "metadata": {
                    "invocationId": "invocation1",
                    "startedOn": "2023-01-01T12:34:56Z",
                    "finishedOn": "2023-01-01T13:34:56Z",
                },
                "byproducts": [
                    {
                        "uri": "https://example.com/byproduct1",
                        "digest": {
                            "algorithm1": "digest1"
                        },
                        "name": "byproduct1",
                        "downloadLocation": "https://example.com/byproduct/download1",
                        "mediaType": "media/type1",
                        "content": "Y29udGVudDE=",
                        "annotations": {
                            "key": "value"
                        },
                    },
                ],
            },
        })
    }

    #[test]
    fn deserialize_slsa_provenance() {
        let json_data = get_test_slsa_provenance_json();
        let deserialized_provenance: SLSAProvenanceV1Predicate =
            serde_json::from_value(json_data).unwrap();
        let expected_provenance = get_test_slsa_provenance();

        assert_eq!(deserialized_provenance, expected_provenance);
    }

    #[test]
    fn serialize_slsa_provenance() {
        let provenance = get_test_slsa_provenance();
        let serialized_provenance = serde_json::to_value(provenance).unwrap();
        let expected_json_data = get_test_slsa_provenance_json();

        assert_eq!(serialized_provenance, expected_json_data);
    }
}
