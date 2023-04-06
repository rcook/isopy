use super::helpers::{
    deserialize_tag_opt, deserialize_version, serialize_tag_opt, serialize_version,
};
use crate::object_model::{Tag, Version};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectRecord {
    #[serde(
        rename = "python_version",
        deserialize_with = "deserialize_version",
        serialize_with = "serialize_version"
    )]
    pub python_version: Version,
    #[serde(
        rename = "tag",
        default,
        //skip_serializing_if = "Option::is_none", // Incompatible with Python isopy!
        deserialize_with = "deserialize_tag_opt",
        serialize_with = "serialize_tag_opt"
    )]
    pub tag: Option<Tag>,
}
