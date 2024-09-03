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
use crate::tng::app_context::AppContext;
use anyhow::Result;
use isopy_lib2::tng::{PackageManager, PackageVersion};
use std::path::Path;

pub struct AppPackageManager {
    ctx: AppContext,
    inner: PackageManager,
}

impl AppPackageManager {
    pub(crate) fn new(ctx: AppContext, inner: PackageManager) -> Self {
        Self { ctx, inner }
    }

    #[allow(unused)]
    pub async fn list_categories(&self) -> Result<()> {
        self.inner.list_categories(&self.ctx).await?;
        Ok(())
    }

    #[allow(unused)]
    pub async fn list_packages(&self) -> Result<()> {
        self.inner.list_packages(&self.ctx).await?;
        Ok(())
    }

    #[allow(unused)]
    pub async fn download_package(&self, version: &PackageVersion) -> Result<()> {
        self.inner.download_package(&self.ctx, version).await?;
        Ok(())
    }

    #[allow(unused)]
    pub async fn install_package(&self, version: &PackageVersion, dir: &Path) -> Result<()> {
        self.inner.install_package(&self.ctx, version, dir).await?;
        Ok(())
    }
}
