use super::helpers::{deserialize_env_name, serialize_env_name};
use crate::object_model::EnvName;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct UseRecord {
    #[serde(rename = "dir")]
    pub dir: PathBuf,
    #[serde(
        rename = "env",
        deserialize_with = "deserialize_env_name",
        serialize_with = "serialize_env_name"
    )]
    pub env_name: EnvName,
}
