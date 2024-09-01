use serde::{self, Deserialize, Deserializer, Serializer};
use url::Url;

pub(crate) fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(url.as_str())
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let url = s.parse().map_err(serde::de::Error::custom)?;
    Ok(url)
}
