use crate::accept::Accept;
use crate::url::Url;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

#[async_trait]
pub trait Context: Sync {
    async fn download(&self, url: &Url, accept: Option<Accept>) -> Result<PathBuf>;
    async fn download_json(&self, url: &Url) -> Result<Value>;
}
