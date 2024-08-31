use crate::accept::Accept;
use crate::url::Url;
use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;

pub trait Context {
    fn download(&self, url: &Url, accept: Option<Accept>) -> Result<PathBuf>;
    fn download_json(&self, url: &Url) -> Result<Value>;
}
