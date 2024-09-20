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
use anyhow::Result;
use async_trait::async_trait;
use isopy_lib::{
    DownloadPackageOptions, GetDirOptionsBuilder, InstallPackageError, InstallPackageOptions,
    ListPackagesOptions, ListTagsOptions, Package, PackageManagerContext, PackageManagerOps,
    PackageSummary, SourceFilter, TagFilter, Tags, UpdateIndexOptions, Version,
};
use std::path::Path;
use std::result::Result as StdResult;
use url::Url;

pub(crate) struct JavaPackageManager {
    ctx: PackageManagerContext,
    url: Url,
}

impl JavaPackageManager {
    pub(crate) fn new(ctx: PackageManagerContext, url: &Url) -> Self {
        Self {
            ctx,
            url: url.clone(),
        }
    }
}

#[async_trait]
impl PackageManagerOps for JavaPackageManager {
    async fn update_index(&self, _options: &UpdateIndexOptions) -> Result<()> {
        todo!()
    }

    async fn list_tags(&self, _options: &ListTagsOptions) -> Result<Tags> {
        todo!()
    }

    async fn list_packages(
        &self,
        _sources: SourceFilter,
        _tags: &TagFilter,
        options: &ListPackagesOptions,
    ) -> Result<Vec<PackageSummary>> {
        let url = self.url.join("example-url")?;
        let options = GetDirOptionsBuilder::default()
            .show_progress(options.show_progress)
            .build()?;
        let dir = self.ctx.get_dir(&url, &options).await?;
        //let adoptium = Adoptium::new(&self.url);
        //adoptium.demo().await?;
        todo!("{dir:?}")
    }

    async fn download_package(
        &self,
        _version: &Version,
        _tags: &TagFilter,
        _options: &DownloadPackageOptions,
    ) -> Result<()> {
        todo!()
    }

    async fn install_package(
        &self,
        _version: &Version,
        _tags: &TagFilter,
        _dir: &Path,
        _options: &InstallPackageOptions,
    ) -> StdResult<Package, InstallPackageError> {
        todo!()
    }
}
