use crate::models::helpers::{b64_option_serde, url_serde};
use serde::de;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use chrono::{DateTime, Utc};

const SLSA_PROVENANCE_URI: &str = "https://slsa.dev/provenance/v1";
const IN_TOTO_STATEMENT_URI: &str = "https://in-toto.io/Statement/v1";

// TODO(mlieberman85): Move in-toto attestation, i.e. non-predicate specific types to their own file.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Algorithm {
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DigestSet(HashMap<Algorithm, String>);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Subject {
    pub name: String,
    pub digest: DigestSet,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ProvenanceAttestation {
    #[serde(
        rename = "_type",
        deserialize_with = "deserialize_in_toto_type",
        serialize_with = "url_serde::serialize"
    )]
    pub _type: Url,
    pub subject: Vec<Subject>,
    #[serde(
        rename = "predicateType",
        deserialize_with = "deserialize_predicate_type",
        serialize_with = "url_serde::serialize"
    )]
    pub predicate_type: Url,
    pub predicate: Predicate,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Predicate {
    #[serde(rename = "buildDefinition")]
    pub build_definition: BuildDefinition,
    #[serde(rename = "runDetails")]
    pub run_details: RunDetails,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BuildDefinition {
    #[serde(rename = "buildType", with = "url_serde")]
    pub build_type: Url,
    #[serde(rename = "externalParameters")]
    pub external_parameters: serde_json::Value,
    #[serde(rename = "internalParameters")]
    pub internal_parameters: serde_json::Value,
    #[serde(rename = "resolvedDependencies")]
    pub resolved_dependencies: Vec<ResourceDescriptor>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RunDetails {
    pub builder: Builder,
    pub metadata: Metadata,
    #[serde(default)]
    pub byproducts: Option<Vec<ResourceDescriptor>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Builder {
    #[serde(with = "url_serde")]
    pub id: Url,
    #[serde(rename = "builderDependencies", default)]
    pub builder_dependencies: Option<Vec<ResourceDescriptor>>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    #[serde(rename = "invocationId")]
    pub invocation_id: String,
    #[serde(rename = "startedOn")]
    pub started_on: DateTime<Utc>,
    #[serde(rename = "finishedOn")]
    pub finished_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ResourceDescriptor {
    #[serde(with = "url_serde")]
    pub uri: Url,
    pub digest: Option<HashMap<String, String>>,
    pub name: Option<String>,
    #[serde(rename = "downloadLocation", with = "url_serde", default)]
    pub download_location: Option<Url>,
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,
    // TODO(mlieberman85): Fix below. Serde was erroring without the default attribute.
    // I think we can probably use a crate with base64 decoding already built in.
    #[serde(with = "b64_option_serde", default)]
    pub content: Option<Vec<u8>>,
    pub annotations: Option<serde_json::Value>,
}

fn deserialize_predicate_type<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value = url_serde::deserialize(deserializer)?;
    let uri = Url::parse(SLSA_PROVENANCE_URI).map_err(de::Error::custom)?;

    if value == uri {
        Ok(value)
    } else {
        Err(de::Error::invalid_value(
            de::Unexpected::Str(value.as_str()),
            &SLSA_PROVENANCE_URI,
        ))
    }
}

fn deserialize_in_toto_type<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value = url_serde::deserialize(deserializer)?;
    let uri = Url::parse(IN_TOTO_STATEMENT_URI).map_err(de::Error::custom)?;

    if value == uri {
        Ok(value)
    } else {
        Err(de::Error::invalid_value(
            de::Unexpected::Str(value.as_str()),
            &IN_TOTO_STATEMENT_URI,
        ))
    }
}

#[test]
fn test_deserialize_provenance_attestation() {
    let example_json = r#"
{
    "_type": "https://in-toto.io/Statement/v1",
    "predicateType": "https://slsa.dev/provenance/v1",
    "predicate": {
        "buildDefinition": {
            "buildType": "https://slsa-framework.github.io/github-actions-buildtypes/workflow/v1",
            "externalParameters": {
                "inputs": {
                    "build_id": 123456768,
                    "deploy_target": "deployment_sys_1a",
                    "perform_deploy": "true"
                },
                "vars": {
                    "MASCOT": "Mona"
                },
                "workflow": {
                    "ref": "refs/heads/main",
                    "repository": "https://github.com/octocat/hello-world",
                    "path": ".github/workflow/release.yml"
                }
            },
            "internalParameters": {
                "github": {
                    "actor_id": "1234567",
                    "event_name": "workflow_dispatch"
                }
            },
            "resolvedDependencies": [
                {
                    "uri": "git+https://github.com/octocat/hello-world@refs/heads/main",
                    "digest": {
                        "gitCommit": "c27d339ee6075c1f744c5d4b200f7901aad2c369"
                    }
                 },
                {
                    "uri": "https://github.com/actions/virtual-environments/releases/tag/ubuntu20/20220515.1"
                }
            ]
        },
        "runDetails": {
            "builder": {
                "id": "https://github.com/slsa-framework/slsa-github-generator/.github/workflows/builder_go_slsa3.yml@refs/tags/v0.0.1"
            },
            "metadata": {
                "invocationId": "https://github.com/octocat/hello-world/actions/runs/1536140711/attempts/1",
                "startedOn": "2023-01-01T12:34:56Z"
            }
        }
    },
    "subject": [
        {
            "name": "_",
            "digest": {
                "sha256": "fe4fe40ac7250263c5dbe1cf3138912f3f416140aa248637a60d65fe22c47da4"
            }
        }
    ]
}
"#;
    let attestation: ProvenanceAttestation = serde_json::from_str(example_json).unwrap();

    // Perform assertions to check if the deserialized data is as expected
    assert_eq!(
        attestation._type,
        Url::parse(IN_TOTO_STATEMENT_URI).unwrap()
    );
    assert_eq!(
        attestation.predicate_type,
        Url::parse(SLSA_PROVENANCE_URI).unwrap()
    );
    assert_eq!(
        attestation.predicate.build_definition.build_type,
        Url::parse("https://slsa-framework.github.io/github-actions-buildtypes/workflow/v1")
            .unwrap()
    );

    assert_eq!(
        attestation.predicate.run_details.metadata.invocation_id,
        "https://github.com/octocat/hello-world/actions/runs/1536140711/attempts/1"
    );

    let external_parameters = &attestation.predicate.build_definition.external_parameters;
    let inputs = external_parameters
        .get("inputs")
        .unwrap()
        .as_object()
        .unwrap();
    let deploy_target = inputs.get("deploy_target").unwrap().as_str().unwrap();
    assert_eq!(deploy_target, "deployment_sys_1a");
}
