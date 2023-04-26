use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;
use url::Url;

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromUrl,
{
    let s = String::deserialize(deserializer)?;
    T::from_url_str(&s).map_err(de::Error::custom)
}

pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsUrlRef,
{
    serializer.serialize_str(value.as_url_ref().as_str())
}

pub trait FromUrl: Sized {
    fn from_url_str(url_str: &str) -> Result<Self, url::ParseError>;
}

pub trait AsUrlRef {
    fn as_url_ref(&self) -> &Url;
}

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
    fn as_url_ref(&self) -> &Url {
        self
    }
}

impl AsUrlRef for Option<Url> {
    fn as_url_ref(&self) -> &Url {
        self.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use url::Url;

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        #[serde(with = "super")]
        pub url: Url,
    }

    #[test]
    fn test_serialize_url() {
        let test_struct = TestStruct {
            url: Url::parse("https://foo.com/bar").unwrap(),
        };
        let serialized = serde_json::to_string(&test_struct).unwrap();

        assert_eq!(serialized, "{\"url\":\"https://foo.com/bar\"}");
    }

    #[test]
    fn test_deserialize_url() {
        let json = serde_json::json!({"url": "https://foo.com"});
        let deserialized: TestStruct = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized.url, Url::parse("https://foo.com").unwrap());
    }

    #[test]
    fn test_deserialize_invalid_url() {
        let json = serde_json::json!({"url": "invalid"});
        let deserialized: Result<TestStruct, _> = serde_json::from_value(json);

        assert!(deserialized.is_err());
    }
}
