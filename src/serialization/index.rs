use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(rename = "tag_name")]
    pub tag: String,
    #[serde(rename = "assets")]
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    #[serde(rename = "browser_download_url")]
    pub uri: String,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: i64,
}
