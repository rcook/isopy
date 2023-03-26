use super::helpers::deserialize_url;
use reqwest::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(rename = "tag_name")]
    pub tag: String,
    #[serde(rename = "assets")]
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    #[serde(rename = "browser_download_url", deserialize_with = "deserialize_url")]
    pub url: Url,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: i64,
}
