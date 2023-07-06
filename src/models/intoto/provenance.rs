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
    /// The parameters that are under external control, such as those set by a user or tenant of the build platform. They MUST be complete at SLSA Build L3, meaning that there is no additional mechanism for an external party to influence the build. (At lower SLSA Build levels, the completeness MAY be best effort.)\nThe build platform SHOULD be designed to minimize the size and complexity of externalParameters, in order to reduce fragility and ease verification. Consumers SHOULD have an expectation of what “good” looks like; the more information that they need to check, the harder that task becomes.\nVerifiers SHOULD reject unrecognized or unexpected fields within externalParameters.
    pub external_parameters:  serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "internalParameters")]
    /// Unordered collection of artifacts needed at build time. Completeness is best effort, at least through SLSA Build L3. For example, if the build script fetches and executes “example.com/foo.sh”, which in turn fetches “example.com/bar.tar.gz”, then both “foo.sh” and “bar.tar.gz” SHOULD be listed here.
    pub internal_parameters: Option<serde_json::Map<String, serde_json::Value>>,

    #[serde(rename = "resolvedDependencies")]
    /// Unordered collection of artifacts needed at build time. Completeness is best effort, at least through SLSA Build L3. For example, if the build script fetches and executes “example.com/foo.sh”, which in turn fetches “example.com/bar.tar.gz”, then both “foo.sh” and “bar.tar.gz” SHOULD be listed here.
    pub resolved_dependencies: Option<Vec<ResourceDescriptor>>,
}

/// A structure representing the run details of the SLSA Provenance v1 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct RunDetails {
    /// Identifies the build platform that executed the invocation, which is trusted to have correctly performed the operation and populated this provenance.
    pub builder: Builder,
    /// metadata about this particular execution of the build.
    pub metadata: Option<BuildMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Additional artifacts generated during the build that are not considered the “output” of the build but that might be needed during debugging or incident response. For example, this might reference logs generated during the build and/or a digest of the fully evaluated build configuration.\nIn most cases, this SHOULD NOT contain all intermediate files generated during the build. Instead, this SHOULD only contain files that are likely to be useful later and that cannot be easily reproduced.
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
pub struct BuildMetadata {
    #[serde(rename = "invocationId")]
    /// Identifies this particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The exact meaning and format is defined by builder.id; by default it is treated as opaque and case-sensitive. The value SHOULD be globally unique.
    pub invocation_id: Option<String>,
    #[serde(rename = "startedOn")]
    /// The timestamp of when the build started.
    pub started_on: Option<DateTime<Utc>>,
    #[serde(rename = "finishedOn")]
    /// The timestamp of when the build completed.
    pub finished_on: Option<DateTime<Utc>>,
}

/// A size-efficient description of any software artifact or resource (mutable or immutable).
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ResourceDescriptor {
    #[serde(with = "url_serde")]
    #[schemars(with = "Url")]
    /// A URI used to identify the resource or artifact globally. This field is REQUIRED unless either digest or content is set.
    pub uri: Url,
    /// A set of cryptographic digests of the contents of the resource or artifact. This field is REQUIRED unless either uri or content is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<HashMap<String, String>>,
    /// Machine-readable identifier for distinguishing between descriptors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "downloadLocation",
        with = "url_serde",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "Url")]
    /// The location of the described resource or artifact, if different from the uri.
    pub download_location: Option<Url>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    /// The MIME Type (i.e., media type) of the described resource or artifact.
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
    /// The contents of the resource or artifact. This field is REQUIRED unless either uri or digest is set.
    #[schemars(with = "String")]
    pub content: Option<Vec<u8>>,
    /// This field MAY be used to provide additional information or metadata about the resource or artifact that may be useful to the consumer when evaluating the attestation against a policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Map<String, serde_json::Value>>,
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
                external_parameters: json!({"key": "value"}).as_object().unwrap().clone(),
                internal_parameters: Some(json!({"key": "value"}).as_object().unwrap().clone()),
                resolved_dependencies: Some(vec![ResourceDescriptor {
                    uri: Url::parse("https://example.com/dependency1").unwrap(),
                    digest: Some(hashmap! {"algorithm1".to_string() => "digest1".to_string()}),
                    name: Some("dependency1".to_string()),
                    download_location: Some(Url::parse("https://example.com/download1").unwrap()),
                    media_type: Some("media/type1".to_string()),
                    content: Some(b"content1".to_vec()),
                    annotations: Some(json!({"key": "value"}).as_object().unwrap().clone()),
                }]),
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
                        annotations: Some(json!({"key": "value"}).as_object().unwrap().clone()),
                    }]),
                    version: Some("1.0.0".to_string()),
                },
                metadata: Some(BuildMetadata {
                    invocation_id: Some("invocation1".to_string()),
                    started_on: Some(DateTime::parse_from_rfc3339("2023-01-01T12:34:56Z")
                        .unwrap()
                        .with_timezone(&Utc)),
                    finished_on: Some(
                        DateTime::parse_from_rfc3339("2023-01-01T13:34:56Z")
                            .unwrap()
                            .with_timezone(&Utc),
                    ),
                }),
                byproducts: Some(vec![ResourceDescriptor {
                    uri: Url::parse("https://example.com/byproduct1").unwrap(),
                    digest: Some(hashmap! {"algorithm1".to_string() => "digest1".to_string()}),
                    name: Some("byproduct1".to_string()),
                    download_location: Some(
                        Url::parse("https://example.com/byproduct/download1").unwrap(),
                    ),
                    media_type: Some("media/type1".to_string()),
                    content: Some(b"content1".to_vec()),
                    annotations: Some(json!({"key": "value"}).as_object().unwrap().clone()),
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
