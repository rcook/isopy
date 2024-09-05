// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::tng::download_options::DownloadOptions;
use anyhow::Result;
use async_trait::async_trait;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use url::Url;

#[async_trait]
pub trait ManagerContextOps: Send + Sync {
    async fn download_file(&self, url: &Url, options: &DownloadOptions) -> Result<PathBuf>;
    async fn get_file(&self, url: &Url) -> Result<PathBuf>;
}

pub struct ManagerContext(Arc<Box<dyn ManagerContextOps>>);

impl ManagerContext {
    pub fn new(inner: Arc<Box<dyn ManagerContextOps>>) -> Self {
        Self(inner)
    }
}

impl Deref for ManagerContext {
    type Target = Arc<Box<dyn ManagerContextOps>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
