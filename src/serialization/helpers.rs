use crate::object_model::{EnvName, LastModified, RepositoryName, Tag, Version};
use reqwest::Url;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_env_name<'de, D>(deserializer: D) -> Result<EnvName, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    EnvName::parse(&s).ok_or(Error::custom("failed to parsed environment name"))
}

pub fn serialize_env_name<S>(x: &EnvName, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn serialize_last_modified<S>(x: &LastModified, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn deserialize_last_modified<'de, D>(deserializer: D) -> Result<LastModified, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(LastModified::parse(&s))
}

pub fn deserialize_repository_name<'de, D>(deserializer: D) -> Result<RepositoryName, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    RepositoryName::parse(&s).ok_or(Error::custom("failed to parse environment name"))
}

pub fn serialize_repository_name<S>(x: &RepositoryName, s: S) -> Result<S::Ok, S::Error>
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

pub fn deserialize_tag_opt<'de, D>(deserializer: D) -> Result<Option<Tag>, D::Error>
where
    D: Deserializer<'de>,
{
    let s_opt: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s_opt {
        Ok(Some(Tag::parse(s)))
    } else {
        Ok(None)
    }
}

pub fn serialize_tag_opt<S>(tag: &Option<Tag>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(ref t) = *tag {
        return s.serialize_str(t.as_str());
    }
    s.serialize_none()
}

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Url::parse(&s).map_err(Error::custom)
}

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
