use crate::object_model::{Tag, Version};
use reqwest::Url;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

pub fn deserialize_tag<'de, D>(deserializer: D) -> Result<Tag, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Tag::parse(&s))
}

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Url::parse(&s).map_err(Error::custom)
}

pub fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Version::parse(&s).ok_or(Error::custom("Failed to parse version"))
}
