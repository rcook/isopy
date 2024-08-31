use crate::url::Url;
use anyhow::Result;
use std::path::PathBuf;

pub trait Context {
    fn download(&self, url: &Url) -> Result<PathBuf>;
}
