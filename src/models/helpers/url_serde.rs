//! Custom (de)serialization functions for `Url` and `Option<Url>` types.
//!
//! This module provides custom serialization and deserialization functions for
//! handling `Url` and `Option<Url>` types, as well as the necessary traits to support
//! these functions.

use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;
use url::Url;

/// Deserializes a URL from a string.
///
/// This function is generic over the type `T` which must implement the `FromUrl` trait.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromUrl,
{
    let s = String::deserialize(deserializer)?;
    T::from_url_str(&s).map_err(de::Error::custom)
}

/// Serializes a URL or an `Option<Url>` as a string.
///
/// This function is generic over the type `T` which must implement the `AsUrlRef` trait.
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsUrlRef,
{
    match value.as_url_ref() {
        Some(url) => serializer.serialize_str(url.as_str()),
        None => serializer.serialize_none(),
    }
}

/// A trait for types that can be created from a URL string.
pub trait FromUrl: Sized {
    fn from_url_str(url_str: &str) -> Result<Self, url::ParseError>;
}

/// A trait for types that can provide a reference to a `Url`.
pub trait AsUrlRef {
    fn as_url_ref(&self) -> Option<&Url>;
}

// Implementations for `Url` and `Option<Url>` types.

impl FromUrl for Url {
    fn from_url_str(url_str: &str) -> Result<Self, url::ParseError> {
        Url::parse(url_str)
    }
}

impl FromUrl for Option<Url> {
    fn from_url_str(url_str: &str) -> Result<Self, url::ParseError> {
        Url::parse(url_str).map(Some)
    }
}

impl AsUrlRef for Url {
    fn as_url_ref(&self) -> Option<&Url> {
        Some(self)
    }
}

impl AsUrlRef for Option<Url> {
    fn as_url_ref(&self) -> Option<&Url> {
        self.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStruct {
        #[serde(with = "super")]
        pub url: Url,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestStructOption {
        #[serde(with = "super", default, skip_serializing_if = "Option::is_none")]
        pub url: Option<Url>,
    }

    #[test]
    fn test_url_serde() {
        let test_data = r#"{"url":"https://example.com/"}"#;
        let test_struct: TestStruct = serde_json::from_str(test_data).unwrap();
        assert_eq!(test_struct.url.as_str(), "https://example.com/");

        let serialized = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(serialized, test_data);
    }

    #[test]
    fn test_url_serde_option_some() {
        let test_data = r#"{"url":"https://example.com/"}"#;
        let test_struct: TestStructOption = serde_json::from_str(test_data).unwrap();
        assert_eq!(
            test_struct.clone().url.unwrap().as_str(),
            "https://example.com/"
        );

        let serialized = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(serialized, test_data);
    }

    #[test]
    fn test_url_serde_option_none() {
        let test_data = r#"{}"#;
        let test_struct: TestStructOption = serde_json::from_str(test_data).unwrap();
        assert!(test_struct.url.is_none());

        let serialized = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(serialized, test_data);
    }
}
