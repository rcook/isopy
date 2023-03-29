use super::helpers::{deserialize_tag, deserialize_url};
use crate::object_model::Tag;
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
    #[serde(rename = "last_modified")]
    pub last_modified: String,
}
