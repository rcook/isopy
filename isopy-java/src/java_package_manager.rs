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
use crate::link_header::LinkHeader;
use anyhow::Result;
use async_trait::async_trait;
use isopy_lib::{
    DownloadPackageOptions, GetDirOptionsBuilder, InstallPackageError, InstallPackageOptions,
    ListPackagesOptions, ListTagsOptions, Package, PackageManagerContext, PackageManagerOps,
    PackageSummary, ProgressIndicator, ProgressIndicatorOptionsBuilder, SourceFilter, TagFilter,
    Tags, UpdateIndexOptions, Version,
};
use reqwest::Client;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use tokio::fs::write;
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

    async fn get_index(&self, show_progress: bool, create_new: bool) -> Result<PathBuf> {
        let url = self.url.join("/v3/info/release_versions")?;
        let dir = self
            .ctx
            .get_dir(
                &url,
                &GetDirOptionsBuilder::default()
                    .show_progress(show_progress)
                    .create_new(create_new)
                    .build()?,
            )
            .await?;
        if !create_new {
            return Ok(dir);
        }

        let client = Client::new();
        let mut request_builder = client.get(url);

        let options = ProgressIndicatorOptionsBuilder::default()
            .enabled(show_progress)
            .build()?;

        let progress_indicator = ProgressIndicator::new(&options)?;
        for i in 0.. {
            progress_indicator.set_message(format!("Downloading page {i}"));

            let response = request_builder.send().await?.error_for_status()?;
            let next_url = LinkHeader::from_response(&response)?.and_then(|x| x.next().clone());
            let bytes = response.bytes().await?;
            let path = dir.join(format!("page-{i:05}"));
            write(path, bytes).await?;
            let Some(next_url) = next_url else {
                return Ok(dir);
            };

            request_builder = client.get(next_url);
        }
        unreachable!()
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
        let dir = self.get_index(options.show_progress, false).await?;
        for e in read_dir(&dir)? {
            let e = e?;
            println!("e={e:?}");
        }
        todo!()
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
