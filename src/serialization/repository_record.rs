use super::helpers::{
    deserialize_repository_name, deserialize_url, serialize_repository_name, serialize_url,
};
use crate::object_model::RepositoryName;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RepositoryRecord {
    #[serde(rename = "github")]
    GitHub {
        #[serde(
            rename = "name",
            deserialize_with = "deserialize_repository_name",
            serialize_with = "serialize_repository_name"
        )]
        name: RepositoryName,

        #[serde(
            rename = "url",
            deserialize_with = "deserialize_url",
            serialize_with = "serialize_url"
        )]
        url: Url,

        #[serde(rename = "enabled")]
        enabled: bool,
    },
    #[serde(rename = "local")]
    Local {
        #[serde(
            rename = "name",
            deserialize_with = "deserialize_repository_name",
            serialize_with = "serialize_repository_name"
        )]
        name: RepositoryName,

        #[serde(rename = "dir")]
        dir: PathBuf,

        #[serde(rename = "enabled")]
        enabled: bool,
    },
}
