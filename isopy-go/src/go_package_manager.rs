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
use crate::api::Release;
use crate::go_package::GoPackage;
use crate::go_version::GoVersion;
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use isopy_lib::{
    dir_url, query, ArchiveType, DownloadFileOptionsBuilder, DownloadPackageOptions,
    GetPackageStateOptions, InstallPackageOptions, ListPackageStatesOptions, ListTagsOptions,
    Package, PackageAvailability, PackageManagerContext, PackageManagerOps, PackageState,
    SourceFilter, TagFilter, Tags, UpdateIndexOptions, Version,
};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;
use tokio::fs::read_to_string;
use url::Url;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const DEFAULT_TAGS: [&str; 2] = ["arm64", "linux"];

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const DEFAULT_TAGS: [&str; 2] = ["amd64", "linux"];

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const DEFAULT_TAGS: [&str; 2] = ["arm64", "darwin"];

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const DEFAULT_TAGS: [&str; 2] = ["amd64", "darwin"];

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
const DEFAULT_TAGS: [&str; 2] = ["arm64", "windows"];

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const DEFAULT_TAGS: [&str; 2] = ["amd64", "windows"];

#[allow(unused)]
macro_rules! downcast_version {
    ($version : expr) => {
        $version
            .as_any()
            .downcast_ref::<$crate::go_version::GoVersion>()
            .ok_or_else(|| ::anyhow::anyhow!("Invalid version type"))?
    };
}

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
        let options = DownloadFileOptionsBuilder::json()
            .update(update)
            .show_progress(show_progress)
            .query(query!([("include", "all"), ("mode", "json")]))
            .build()?;
        let url = dir_url(&self.url);
        let path = self.ctx.download_file(&url, &options).await?;
        let s = read_to_string(path).await?;
        let index = serde_json::from_str(&s)?;
        Ok(index)
    }

    async fn get_packages(&self, update: bool, show_progress: bool) -> Result<Vec<GoPackage>> {
        let index = self.get_index(update, show_progress).await?;
        let mut packages = Vec::new();
        for release in serde_json::from_value::<Vec<Release>>(index)? {
            let filter_tags = HashSet::from(DEFAULT_TAGS);
            for file in release.files {
                match file.kind.as_str() {
                    "archive" => {
                        let tags = [file.arch.as_str(), file.os.as_str()]
                            .into_iter()
                            .collect::<HashSet<_>>();
                        if tags.is_superset(&filter_tags) {
                            let version = file.version.parse::<GoVersion>()?;
                            let url = self.url.join(&file.file_name)?;
                            let (kind, path) = match self.ctx.get_file(&url).await {
                                Ok(p) => (PackageAvailability::Local, Some(p)),
                                _ => (PackageAvailability::Remote, None),
                            };
                            let (archive_type, _) = ArchiveType::strip_suffix(&file.file_name)
                                .ok_or_else(|| {
                                    anyhow!(
                                        "Cannot determine archive type of file name {}",
                                        file.file_name
                                    )
                                })?;
                            let checksum = file.sha256.parse()?;
                            let tags = vec![file.arch, file.os];
                            packages.push(GoPackage::new(
                                &file.file_name,
                                kind,
                                archive_type,
                                &url,
                                &version,
                                &path,
                                checksum,
                                tags,
                            ));
                        }
                    }
                    "installer" | "source" => {}
                    _ => todo!("Unimplemented file kind {}", file.kind),
                }
            }
        }
        Ok(packages)
    }

    async fn get_package_inner(
        &self,
        update: bool,
        show_progress: bool,
        version: &GoVersion,
        _tags: &TagFilter,
    ) -> Result<GoPackage> {
        let packages = self.get_packages(update, show_progress).await?;
        let mut packages = packages
            .into_iter()
            .filter(|p| p.version() == version)
            .collect::<Vec<_>>();
        if packages.is_empty() {
            bail!("No matching packages found")
        }

        packages.sort_by_cached_key(|p| p.version().clone());
        packages.reverse();
        Ok(packages
            .into_iter()
            .next()
            .expect("Vector must contain at least one element"))
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

    async fn list_package_states(
        &self,
        sources: SourceFilter,
        _tags: &TagFilter, // TBD
        options: &ListPackageStatesOptions,
    ) -> Result<Vec<PackageState>> {
        use isopy_lib::SourceFilter::*;
        let filter_tags = DEFAULT_TAGS
            .into_iter()
            .map(String::from)
            .collect::<HashSet<_>>();
        let mut packages = Vec::new();
        for package in self.get_packages(false, options.show_progress).await? {
            if package.tags().is_superset(&filter_tags)
                && matches!(
                    (
                        sources,
                        package.availability() == PackageAvailability::Local
                    ),
                    (All, _) | (Local, true) | (Remote, false)
                )
            {
                packages.push(package);
            }
        }

        packages.sort_by(|a, b| a.version().cmp(b.version()));
        packages.reverse();

        Ok(packages
            .into_iter()
            .map(GoPackage::into_package_info)
            .collect::<Vec<_>>())
    }

    async fn get_package_state(
        &self,
        version: &Version,
        tags: &TagFilter,
        options: &GetPackageStateOptions,
    ) -> Result<Option<PackageState>> {
        let version = downcast_version!(version);
        let package = self
            .get_package_inner(false, options.show_progress, version, tags)
            .await?;
        Ok(Some(package.into_package_info()))
    }

    async fn download_package(
        &self,
        version: &Version,
        tags: &TagFilter,
        options: &DownloadPackageOptions,
    ) -> Result<()> {
        let version = downcast_version!(version);
        let package = self
            .get_package_inner(false, options.show_progress, version, tags)
            .await?;
        let options = DownloadFileOptionsBuilder::default()
            .update(false)
            .checksum(Some(package.checksum().clone()))
            .show_progress(options.show_progress)
            .build()?;
        _ = self.ctx.download_file(package.url(), &options).await?;
        Ok(())
    }

    async fn install_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        dir: &Path,
        options: &InstallPackageOptions,
    ) -> Result<Package> {
        let version = downcast_version!(version);
        let Ok(package) = self
            .get_package_inner(false, options.show_progress, version, tag_filter)
            .await
        else {
            let tags = tag_filter.tags(&[]);
            bail!("No release {version} with tags {tags:?} found");
        };

        let Ok(path) = self.ctx.get_file(package.url()).await else {
            let tags = tag_filter.tags(&[]);
            bail!("Failed to download release {version} with tags {tags:?}");
        };

        package.archive_type().unpack(&path, dir, options).await?;

        Ok(Package::new(package))
    }
}
