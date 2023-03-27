use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct EnvRecord {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "python_dir")]
    pub python_dir: PathBuf,
    #[serde(rename = "python_version")]
    pub python_version: String,
    #[serde(rename = "tag")]
    pub tag: String,
}
