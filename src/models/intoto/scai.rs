//! SCAI predicate model and associated structures.
//!
//! This module provides structs for the SCAIV02Predicate.
//! It also includes the necessary (de)serialization code for handling the SCAI predicate.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::provenance::ResourceDescriptor;

/// This is based on the model in: 
/// {
///     "predicateType": "https://in-toto.io/attestation/scai/attribute-report/v0.2",
///     "predicate": {
///         "attributes": [{
///             "attribute": "<ATTRIBUTE>",
///             "target": { [ResourceDescriptor] }, // optional
///             "conditions": { /* object */ }, // optional
///             "evidence": { [ResourceDescriptor] } // optional
///         }],
///         "producer": { [ResourceDescriptor] } // optional
///     }
/// }

/// A structure representing the SLSA Provenance v1 Predicate.
#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct SCAIV02Predicate {
    pub attributes: Vec<Attribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<ResourceDescriptor>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Attribute {
    pub attribute: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ResourceDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<ResourceDescriptor>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_scaiv02_predicate_serialization() {
        let target_resource_descriptor = ResourceDescriptor {
            uri: Url::parse("http://target.example.com/").unwrap(),
            digest: None,
            name: Some("TargetResource".into()),
            download_location: None,
            media_type: Some("application/json".into()),
            content: None,
            annotations: None,
        };

        let evidence_resource_descriptor = ResourceDescriptor {
            uri: Url::parse("http://evidence.example.com/").unwrap(),
            digest: None,
            name: Some("EvidenceResource".into()),
            download_location: None,
            media_type: Some("application/json".into()),
            content: None,
            annotations: None,
        };

        let producer_resource_descriptor = ResourceDescriptor {
            uri: Url::parse("http://producer.example.com/").unwrap(),
            digest: None,
            name: Some("ProducerResource".into()),
            download_location: None,
            media_type: Some("application/json".into()),
            content: None,
            annotations: None,
        };

        let attribute = Attribute {
            attribute: "TestAttribute".into(),
            target: Some(target_resource_descriptor),
            conditions: Some({
                let mut map = HashMap::new();
                map.insert("condition1".into(), "value1".into());
                map
            }),
            evidence: Some(evidence_resource_descriptor),
        };

        let predicate = SCAIV02Predicate {
            attributes: vec![attribute],
            producer: Some(producer_resource_descriptor),
        };

        let serialized = serde_json::from_str::<serde_json::Value>(serde_json::to_string(&predicate).unwrap().as_str()).unwrap();
        let expected = serde_json::from_str::<serde_json::Value>(r#"{
            "attributes": [
                {
                    "attribute": "TestAttribute",
                    "target": { "uri": "http://target.example.com/", "name": "TargetResource", "mediaType": "application/json" },
                    "conditions": { "condition1": "value1" },
                    "evidence": { "uri": "http://evidence.example.com/", "name": "EvidenceResource", "mediaType": "application/json" }
                }
            ],
            "producer": { "uri": "http://producer.example.com/", "name": "ProducerResource", "mediaType": "application/json" }
        }"#).unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_scaiv02_predicate_deserialization() {
        let data = r#"{
            "attributes": [
                {
                    "attribute": "TestAttribute",
                    "target": { "uri": "http://target.example.com", "name": "TargetResource", "mediaType": "application/json" },
                    "conditions": { "condition1": "value1" },
                    "evidence": { "uri": "http://evidence.example.com", "name": "EvidenceResource", "mediaType": "application/json" }
                }
            ],
            "producer": { "uri": "http://producer.example.com", "name": "ProducerResource", "mediaType": "application/json" }
        }"#;
        let deserialized: SCAIV02Predicate = serde_json::from_str(data).unwrap();
        assert_eq!(deserialized.attributes[0].attribute, "TestAttribute");
        assert_eq!(deserialized.attributes[0].conditions.as_ref().unwrap().get("condition1"), Some(&"value1".to_string()));
        assert_eq!(deserialized.attributes[0].target.as_ref().unwrap().name, Some("TargetResource".into()));
        assert_eq!(deserialized.attributes[0].evidence.as_ref().unwrap().name, Some("EvidenceResource".into()));
        assert_eq!(deserialized.producer.as_ref().unwrap().name, Some("ProducerResource".into()));
    }
}