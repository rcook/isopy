use crate::accept::Accept;
use crate::url::Url;
use anyhow::Result;
use std::path::PathBuf;

pub trait Context {
    fn download(&self, url: &Url, accept: Option<Accept>) -> Result<PathBuf>;
}
