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
    DownloadFileOptions, DownloadPackageOptions, InstallPackageError, InstallPackageOptions,
    ListPackagesOptions, ListTagsOptions, Package, PackageManagerContext, PackageManagerOps,
    PackageSummary, SourceFilter, TagFilter, Tags, UpdateIndexOptions, Version,
};
use serde_json::Value;
use std::path::Path;
use std::result::Result as StdResult;
use tokio::fs::read_to_string;
use url::Url;

pub(crate) struct GoPackageManager {
    ctx: PackageManagerContext,
    url: Url,
}

impl GoPackageManager {
    pub(crate) fn new(ctx: PackageManagerContext, url: &Url) -> Self {
        Self {
            ctx,
            url: url.clone(),
        }
    }

    async fn get_index(&self, update: bool, show_progress: bool) -> Result<Value> {
        let options = DownloadFileOptions::json()
            .update(update)
            .show_progress(show_progress)
            .query(&[("include", "all"), ("mode", "json")]);
        //TBD: let url = dir_url(&self.url);
        let path = self.ctx.download_file(&self.url, &options).await?;
        let s = read_to_string(path).await?;
        let index = serde_json::from_str(&s)?;
        Ok(index)
    }
}

#[async_trait]
impl PackageManagerOps for GoPackageManager {
    async fn update_index(&self, options: &UpdateIndexOptions) -> Result<()> {
        self.get_index(true, options.show_progress).await?;
        Ok(())
    }

    async fn list_tags(&self, _options: &ListTagsOptions) -> Result<Tags> {
        todo!()
    }

    async fn list_packages(
        &self,
        _sources: SourceFilter,
        _tags: &TagFilter,
        _options: &ListPackagesOptions,
    ) -> Result<Vec<PackageSummary>> {
        Ok(vec![])
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
