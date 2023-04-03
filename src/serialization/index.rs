use super::helpers::{
    deserialize_last_modified, deserialize_tag, deserialize_url, serialize_last_modified,
};
use crate::object_model::{LastModified, Tag};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PackageRecord {
    #[serde(rename = "tag_name", deserialize_with = "deserialize_tag")]
    pub tag: Tag,
    #[serde(rename = "assets")]
    pub assets: Vec<AssetRecord>,
}

#[derive(Debug, Deserialize)]
pub struct AssetRecord {
    #[serde(rename = "browser_download_url", deserialize_with = "deserialize_url")]
    pub url: Url,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexRecord {
    #[serde(
        rename = "last_modified",
        deserialize_with = "deserialize_last_modified",
        serialize_with = "serialize_last_modified"
    )]
    pub last_modified: LastModified,
}
