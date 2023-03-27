use super::AssetMeta;
use reqwest::Url;

pub struct Asset {
    pub name: String,
    pub url: Url,
    pub meta: AssetMeta,
}
