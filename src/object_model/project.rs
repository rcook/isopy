use crate::object_model::{Tag, Version};
use std::path::PathBuf;

pub struct Project {
    pub config_path: PathBuf,
    pub python_version: Version,
    pub tag: Option<Tag>,
}
