use super::helpers::{deserialize_last_modified, serialize_last_modified};
use crate::object_model::LastModified;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexRecord {
    #[serde(
        rename = "last_modified",
        deserialize_with = "deserialize_last_modified",
        serialize_with = "serialize_last_modified"
    )]
    pub last_modified: LastModified,
}
