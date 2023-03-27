use crate::object_model::{EnvName, Tag, Version};
use reqwest::Url;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_env_name<'de, D>(deserializer: D) -> Result<EnvName, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    EnvName::parse(&s).ok_or(Error::custom("Failed to environment name"))
}

pub fn serialize_env_name<S>(x: &EnvName, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn deserialize_tag<'de, D>(deserializer: D) -> Result<Tag, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Tag::parse(&s))
}

pub fn serialize_tag<S>(x: &Tag, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Url::parse(&s).map_err(Error::custom)
}

#[allow(unused)]
pub fn serialize_url<S>(x: &Url, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Version::parse(&s).ok_or(Error::custom("Failed to parse version"))
}

pub fn serialize_version<S>(x: &Version, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}
