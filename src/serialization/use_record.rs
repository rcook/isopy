use super::helpers::{deserialize_environment_name, serialize_environment_name};
use crate::object_model::EnvironmentName;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct UseRecord {
    #[serde(rename = "dir")]
    pub dir: PathBuf,
    #[serde(
        rename = "env",
        deserialize_with = "deserialize_environment_name",
        serialize_with = "serialize_environment_name"
    )]
    pub environment_name: EnvironmentName,
}
