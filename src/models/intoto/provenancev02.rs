//! SLSA provenance predicate model and associated structures.
//!
//! This module provides structs for the SLSAProvenanceV02Predicate and its related structures.
//! It also includes the necessary (de)serialization code for handling SLSA provenance predicates.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/// A structure representing the SLSA Provenance v0.2 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct SLSAProvenanceV02Predicate {
    /// The entity that executed the invocation, which is trusted to have correctly performed the operation and populated this provenance.
    pub builder: Builder,
    #[serde(rename = "buildType")]
    /// The type of build that was performed.
    pub build_type: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The event that kicked off the build.
    pub invocation: Option<Invocation>,
    #[serde(rename = "buildConfig", skip_serializing_if = "Option::is_none")]
    /// The steps in the build. If invocation.configSource is not available, buildConfig can be used to verify information about the build.
    pub build_config: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Metadata about this particular execution of the build.
    pub metadata: Option<BuildMetadata>,
    #[serde(rename = "materials", skip_serializing_if = "Option::is_none")]
    /// Unordered collection of artifacts that influenced the build including sources, dependencies, build tools, base images, and so on. Completeness is best effort, at least through SLSA Build L3. For example, if the build script fetches and executes “example.com/foo.sh”, which in turn fetches “example.com/bar.tar.gz”, then both “foo.sh” and “bar.tar.gz” SHOULD be listed here.
    pub materials: Option<Vec<ResourceDescriptor>>,
}

/// A structure representing the builder information of the SLSA Provenance v0.2 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Builder {
    pub id: Url
}

/// A structure identifying the event that kicked off the build in the SLSA Provenance v0.2 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Invocation {
    #[serde(rename = "configSource", skip_serializing_if = "Option::is_none")]
    /// Description of where the config file that kicked off the build came from. This is effectively a pointer to the source where buildConfig came from.
    pub config_source: Option<ConfigSource>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    /// Collection of all external inputs that influenced the build on top of invocation.configSource.
    pub parameters: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    /// Any other builder-controlled inputs necessary for correctly evaluating the build. Usually only needed for reproducing the build but not evaluated as part of policy.
    pub environment: Option<serde_json::Map<String, serde_json::Value>>,

}

/// A structure representing the description of where the config file that kicked off the build came from in the SLSA Provenance v0.2 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ConfigSource {
    /// The identity of the source of the config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<Url>,
    /// A set of cryptographic digests of the contents of the resource or artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<HashMap<String, String>>,
    /// The entry point into the build. This is often a path to a configuration file and/or a target label within that file.
    #[serde(rename = "entryPoint", skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,
}

/// A structure representing the metadata of the SLSA Provenance v0.2 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct BuildMetadata {
    #[serde(rename = "buildInvocationId", skip_serializing_if = "Option::is_none")]
    /// Identifies this particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The exact meaning and format is defined by builder.id; by default it is treated as opaque and case-sensitive. The value SHOULD be globally unique.
    pub invocation_id: Option<String>,
    #[serde(rename = "buildStartedOn", skip_serializing_if = "Option::is_none")]
    /// The timestamp of when the build started.
    pub started_on: Option<DateTime<Utc>>,
    #[serde(rename = "buildFinishedOn", skip_serializing_if = "Option::is_none")]
    /// The timestamp of when the build completed.
    pub finished_on: Option<DateTime<Utc>>,
    #[serde(rename = "completeness", skip_serializing_if = "Option::is_none")]
    /// Information on how complete the provided information is.
    pub completeness: Option<Completeness>,
    #[serde(rename = "reproducible", skip_serializing_if = "Option::is_none")]
    /// Whether the builder claims that running invocation on materials will produce bit-for-bit identical output.
    pub reproducible: Option<bool>,
 }

/// A structure representing the completeness claims of the SLSA Provenance v0.2 Predicate.
 #[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
 pub struct Completeness {
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    /// Whether the builder claims that nvocation.parameters is complete, meaning that all external inputs are properly captured in invocation.parameters.
    pub parameters: Option<bool>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    /// Whether the builder claims that invocation.environment is complete.
    pub environment: Option<bool>,
    #[serde(rename = "materials", skip_serializing_if = "Option::is_none")]
    /// Whether the builder claims that materials is complete, usually through some controls to prevent network access.
    pub materials: Option<bool>,
}

/// A size-efficient description of any software artifact or resource (mutable or immutable).
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ResourceDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A URI used to identify the resource or artifact globally. This field is REQUIRED unless digest is set.
    pub uri: Option<Url>,
    /// A set of cryptographic digests of the contents of the resource or artifact. This field is REQUIRED unless uri is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use serde_json::json;

    fn get_test_slsa_provenance() -> SLSAProvenanceV02Predicate {
        SLSAProvenanceV02Predicate {
            builder: Builder {
                id: Url::parse("https://example.com/builder/v1").unwrap(),
            },
            build_type: Url::parse("https://example.com/buildType/v1").unwrap(),
            invocation: Some(Invocation {
                config_source: Some(ConfigSource {
                    uri: Some(Url::parse("https://example.com/source1").unwrap()),
                    digest: Some(hashmap! {"algorithm1".to_string() => "digest1".to_string()}),
                    entry_point: Some("myentrypoint".to_string()),
                }),
                parameters: Some(json!({"key": "value"}).as_object().unwrap().clone()),
                environment: Some(json!({"key": "value"}).as_object().unwrap().clone()),
            }),
            build_config: Some(json!({"key": "value"}).as_object().unwrap().clone()),
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
                completeness: Some(Completeness {
                    parameters: Some(true),
                    environment: Some(true),
                    materials: Some(true),
                }),
                reproducible: Some(false),
            }),
            materials: Some(vec![ResourceDescriptor {
                uri: Some(Url::parse("https://example.com/material1").unwrap()),
                digest: Some(hashmap! {"algorithm1".to_string() => "digest1".to_string()}),
            }]),
        }
    }

    fn get_test_slsa_provenance_json() -> serde_json::Value {
        json!({
            "builder": {
                "id": "https://example.com/builder/v1",
            },
            "buildType": "https://example.com/buildType/v1",
            "invocation": {
                "configSource": {
                    "uri": "https://example.com/source1",
                    "digest": {
                        "algorithm1": "digest1"
                    },
                "entryPoint": "myentrypoint"
                },
                "parameters": {
                    "key": "value"
                },
                "environment": {
                    "key": "value"
                }
            },
            "buildConfig": {
                "key": "value",
            },
            "metadata": {
                "buildInvocationId": "invocation1",
                "buildStartedOn": "2023-01-01T12:34:56Z",
                "buildFinishedOn": "2023-01-01T13:34:56Z",
                "completeness": {
                    "parameters": true,
                    "environment": true,
                    "materials": true
                },
                "reproducible": false
            },
            "materials": [
                {
                    "uri": "https://example.com/material1",
                    "digest": {
                        "algorithm1": "digest1"
                    }
                }
            ]
        })
    }

    #[test]
    fn deserialize_slsa_provenance() {
        let json_data = get_test_slsa_provenance_json();
        let deserialized_provenance: SLSAProvenanceV02Predicate =
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
