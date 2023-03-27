use super::helpers::{deserialize_tag, deserialize_version};
use crate::object_model::{Tag, Version};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct EnvRecord {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "python_dir")]
    pub python_dir: PathBuf,
    #[serde(rename = "python_version", deserialize_with = "deserialize_version")]
    pub python_version: Version,
    #[serde(rename = "tag", deserialize_with = "deserialize_tag")]
    pub tag: Tag,
}
