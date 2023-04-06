use super::helpers::{deserialize_tag, deserialize_version, serialize_tag, serialize_version};
use crate::object_model::{Tag, Version};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectEnvironmentRecord {
    #[serde(rename = "dir_config_path")]
    pub config_path: PathBuf,
    #[serde(rename = "python_dir")]
    pub python_dir_rel: PathBuf,
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
