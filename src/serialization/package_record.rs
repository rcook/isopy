use super::helpers::deserialize_tag;
use super::AssetRecord;
use crate::object_model::Tag;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageRecord {
    #[serde(rename = "tag_name", deserialize_with = "deserialize_tag")]
    pub tag: Tag,
    #[serde(rename = "assets")]
    pub assets: Vec<AssetRecord>,
}
