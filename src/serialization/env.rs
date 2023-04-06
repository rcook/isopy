use super::helpers::{
    deserialize_env_name, deserialize_tag, deserialize_version, serialize_env_name, serialize_tag,
    serialize_version,
};
use crate::object_model::{EnvironmentName, Tag, Version};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct NamedEnvRecord {
    #[serde(
        rename = "name",
        deserialize_with = "deserialize_env_name",
        serialize_with = "serialize_env_name"
    )]
    pub name: EnvironmentName,
    #[serde(rename = "python_dir")]
    pub python_dir: PathBuf,
    #[serde(
        rename = "python_version",
        deserialize_with = "deserialize_version",
        serialize_with = "serialize_version"
    )]
    pub python_version: Version,
    #[serde(
        rename = "tag",
        deserialize_with = "deserialize_tag",
        serialize_with = "serialize_tag"
    )]
    pub tag: Tag,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousEnvRecord {
    #[serde(rename = "dir_config_path")]
    pub config_path: PathBuf,
    #[serde(rename = "python_dir")]
    pub python_dir: PathBuf,
    #[serde(
        rename = "python_version",
        deserialize_with = "deserialize_version",
        serialize_with = "serialize_version"
    )]
    pub python_version: Version,
    #[serde(
        rename = "tag",
        deserialize_with = "deserialize_tag",
        serialize_with = "serialize_tag"
    )]
    pub tag: Tag,
}
