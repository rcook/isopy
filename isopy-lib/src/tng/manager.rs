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
use crate::tng::version::Version;
use anyhow::Result;
use async_trait::async_trait;
use std::ops::Deref;
use std::path::Path;

#[async_trait]
pub trait ManagerOps: Send + Sync {
    async fn update_index(&self) -> Result<()>;
    async fn list_categories(&self) -> Result<()>;
    async fn list_packages(&self) -> Result<()>;
    async fn download_package(&self, version: &Version) -> Result<()>;
    async fn install_package(&self, version: &Version, dir: &Path) -> Result<()>;
}

pub struct Manager(Box<dyn ManagerOps>);

impl Manager {
    pub fn new(inner: Box<dyn ManagerOps>) -> Self {
        Self(inner)
    }
}

impl Deref for Manager {
    type Target = Box<dyn ManagerOps>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
