use super::AssetMeta;
use reqwest::Url;

#[derive(Debug, PartialEq)]
pub struct Asset {
    pub name: String,
    pub url: Url,
    pub meta: AssetMeta,
}
