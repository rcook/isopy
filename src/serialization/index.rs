use super::helpers::deserialize_url;
use reqwest::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageRecord {
    #[serde(rename = "tag_name")]
    pub tag: String,
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
