use crate::tng::download_options::DownloadOptions;
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use url::Url;

#[async_trait]
pub trait Context: Sync {
    async fn download_file(&self, url: &Url, options: &DownloadOptions) -> Result<PathBuf>;
    async fn get_file(&self, url: &Url) -> Result<PathBuf>;
}
