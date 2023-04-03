use super::{AssetMeta, Tag};
use reqwest::Url;

#[derive(Debug, PartialEq)]
pub struct Asset {
    pub name: String,
    pub tag: Tag,
    pub url: Url,
    pub size: i64,
    pub meta: AssetMeta,
}
