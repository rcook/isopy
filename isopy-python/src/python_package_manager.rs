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
use crate::checksum::get_checksum;
use crate::choose_best::choose_best;
use crate::constants::PLATFORM_TAGS;
use crate::index_item::IndexItem;
use crate::local_package_info::LocalPackageInfo;
use crate::package_cache::read_package_cache;
use crate::python_package::PythonPackage;
use crate::python_version::PythonVersion;
use anyhow::{Result, bail};
use async_trait::async_trait;
use isopy_lib::{
    Accept, DownloadAssetOptionsBuilder, DownloadPackageOptions,
    DownloadPaginatedAssetOptionsBuilder, DownloadPaginatedAssetResponse, GetPackageOptions,
    InstallPackageOptions, ListPackagesOptions, ListTagsOptions, Package, PackageInfo,
    PackageManagerContext, PackageManagerOps, SourceFilter, TagFilter, Tags, UpdateIndexOptions,
    Version, error_for_github_rate_limit,
};
use serde_json::Value;
use std::borrow::ToOwned;
use std::collections::HashSet;
use std::fs::{metadata, read_to_string};
use std::path::Path;
use url::Url;

const PACKAGE_CACHE_FILE_NAME: &str = "packages.yaml";

macro_rules! downcast_version {
    ($version : expr) => {
        $version
            .as_any()
            .downcast_ref::<$crate::python_version::PythonVersion>()
            .ok_or_else(|| ::anyhow::anyhow!("Invalid version type"))?
    };
}

pub(crate) struct PythonPackageManager {
    ctx: PackageManagerContext,
    moniker: String,
    url: Url,
    platform_tags: HashSet<String>,
}

impl PythonPackageManager {
    pub(crate) fn new(ctx: PackageManagerContext, moniker: &str, url: &Url) -> Self {
        Self {
            ctx,
            moniker: moniker.to_owned(),
            url: url.to_owned(),
            platform_tags: PLATFORM_TAGS
                .iter()
                .copied()
                .map(ToOwned::to_owned)
                .collect::<HashSet<_>>(),
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn on_after_install(dir: &Path) -> Result<()> {
        use log::trace;
        use std::os::unix::fs::symlink;

        let bin_dir = dir.join("bin");
        let link = bin_dir.join("python");
        if !link.exists() {
            let original = bin_dir.join("python3");
            trace!("Creating link {} to {}", link.display(), original.display());
            symlink(&original, &link)?;
            trace!("Created link {} to {}", link.display(), original.display());
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn on_after_install(dir: &Path) -> Result<()> {
        use log::trace;
        use std::fs::write;

        let cmd_path = dir.join("python3.cmd");
        if !cmd_path.exists() {
            const WRAPPER: &str = "@echo off\n\"%~dp0python.exe\" %*\n";
            trace!("Creating wrapper script {}", cmd_path.display());
            write(&cmd_path, WRAPPER)?;
            trace!("Created wrapper script {}", cmd_path.display());
        }
        Ok(())
    }

    fn make_package_info(info: LocalPackageInfo) -> PackageInfo {
        PackageInfo::new(
            info.package.metadata.name.clone(),
            &info.package.url,
            Version::new(info.package.metadata.version.clone()),
            info.path,
        )
    }

    #[allow(clippy::unused_async)]
    async fn get_index_response(
        &self,
        update: bool,
        show_progress: bool,
    ) -> Result<DownloadPaginatedAssetResponse> {
        /*
        fn from_dir(dir: &Path) -> Result<DownloadPaginatedAssetResponse> {
            let mut parts = Vec::new();
            for entry in read_dir(dir)
                .with_context(|| format!("directory {dir} not found", dir = dir.display()))?
            {
                let entry = entry?;
                if let Some(s) = entry.file_name().to_str()
                    && let Some(suffix) = s.strip_prefix(PAGINATION_PART_PREFIX)
                    && let Ok(index) = suffix.parse::<usize>()
                {
                    parts.push((entry.path(), index));
                }
            }

            if parts.is_empty() {
                bail!("no paginated download found at {dir}", dir = dir.display())
            }

            parts.sort_by(|(_, index_a), (_, index_b)| index_a.cmp(index_b));

            Ok(DownloadPaginatedAssetResponse {
                dir: dir.to_path_buf(),
                parts: parts.into_iter().map(|p| p.0).collect(),
            })
        }
        */

        let url = {
            let mut url = self.url.clone();
            {
                let mut pairs = url.query_pairs_mut();
                pairs.append_pair("per_page", "10");
                pairs.append_pair("page", "1");
            }
            url
        };

        let options = DownloadPaginatedAssetOptionsBuilder::default()
            .show_progress(show_progress)
            .update(update)
            .accept(Some(Accept::ApplicationGitHubJson))
            .check(Some(error_for_github_rate_limit))
            .build()?;
        self.ctx.download_paginated_asset(&url, &options).await

        //from_dir(Path::new("downloads/transaction"))
    }

    #[allow(unused)]
    async fn fetch_packages(
        &self,
        _cache_path: &Path,
        show_progress: bool,
    ) -> Result<Vec<PythonPackage>> {
        fn get_packages(
            response: &DownloadPaginatedAssetResponse,
            platform_tags: &HashSet<String>,
        ) -> Result<Vec<PythonPackage>> {
            fn read_values_in_order(
                response: &DownloadPaginatedAssetResponse,
            ) -> Result<Vec<Value>> {
                fn from_value(value: Value) -> Option<(String, Value)> {
                    let obj = value.as_object()?;
                    let tag_name = obj.get("tag_name")?;
                    let tag = tag_name.as_str()?;
                    Some((String::from(tag), value))
                }

                let mut values = Vec::new();
                for p in &response.parts {
                    let s = read_to_string(p)?;
                    let temp = serde_json::from_str::<Vec<Value>>(&s)?;
                    values.extend(temp.into_iter().filter_map(from_value));
                }

                values.sort_by(|a, b| b.0.cmp(&a.0));

                Ok(values.into_iter().map(|(_, value)| value).collect())
            }

            fn into_packages(
                response: &DownloadPaginatedAssetResponse,
                platform_tags: &HashSet<String>,
            ) -> Result<Vec<PythonPackage>> {
                let mut packages = Vec::new();
                for value in read_values_in_order(response)? {
                    packages.extend(
                        PythonPackage::read_all(&IndexItem::new(&value))?
                            .into_iter()
                            .filter_map(|p| {
                                if p.metadata.tags.is_superset(platform_tags) {
                                    Some(p)
                                } else {
                                    None
                                }
                            }),
                    );
                }

                Ok(packages)
            }

            into_packages(response, platform_tags)
        }

        let response = self.get_index_response(false, show_progress).await?;
        let packages = get_packages(&response, &self.platform_tags)?;
        //write_package_cache(cache_path, &packages)?;
        Ok(packages)
    }

    async fn read_packages(&self, show_progress: bool) -> Result<Vec<PythonPackage>> {
        let package_cache_path = self.ctx.base_dir().join(PACKAGE_CACHE_FILE_NAME);
        if !package_cache_path.exists() {
            return self
                .fetch_packages(&package_cache_path, show_progress)
                .await;
        }

        let Some(index_path) = self.ctx.check_asset(&self.url)? else {
            return read_package_cache(&package_cache_path);
        };

        let index_created_at = metadata(&index_path)?.created()?;
        let cache_created_at = metadata(&package_cache_path)?.created()?;
        if index_created_at > cache_created_at {
            return self
                .fetch_packages(&package_cache_path, show_progress)
                .await;
        }

        read_package_cache(&package_cache_path)
    }

    fn filter_packages(
        &self,
        packages: Vec<PythonPackage>,
        source_filter: SourceFilter,
        version: Option<&PythonVersion>,
        tag_filter: &TagFilter,
    ) -> Result<Vec<LocalPackageInfo>> {
        use isopy_lib::SourceFilter::{All, Local, Remote};

        let tags = tag_filter
            .tags
            .iter()
            .map(ToOwned::to_owned)
            .collect::<HashSet<_>>();
        let mut infos = Vec::new();
        for package in packages {
            let m = &package.metadata;
            let version_matches = match version {
                Some(version) => m.version.matches(version),
                None => true,
            };
            if version_matches && m.tags.is_superset(&tags) {
                let path = self.ctx.check_asset(&package.url)?;
                if matches!(
                    (source_filter, path.is_some()),
                    (All, _) | (Local, true) | (Remote, false)
                ) {
                    infos.push(LocalPackageInfo { package, path });
                }
            }
        }

        // Ensure there is exactly one matching package for a given version and build label
        Ok(choose_best(infos))
    }

    async fn read_package(
        &self,
        version: &PythonVersion,
        tag_filter: &TagFilter,
        show_progress: bool,
    ) -> Result<Option<LocalPackageInfo>> {
        Ok(self
            .filter_packages(
                self.read_packages(show_progress).await?,
                SourceFilter::All,
                Some(version),
                tag_filter,
            )?
            .into_iter()
            .next())
    }
}

#[async_trait]
impl PackageManagerOps for PythonPackageManager {
    async fn update_index(&self, options: &UpdateIndexOptions) -> Result<()> {
        self.get_index_response(true, options.show_progress).await?;
        Ok(())
    }

    async fn list_tags(&self, options: &ListTagsOptions) -> Result<Tags> {
        let mut other_tags = HashSet::new();
        for package in self.read_packages(options.show_progress).await? {
            other_tags.extend(package.metadata.tags.iter().map(ToOwned::to_owned));
        }

        other_tags.retain(|t| !self.platform_tags.contains(t));
        let mut other_tags = other_tags.into_iter().collect::<Vec<_>>();
        other_tags.sort();
        let other_tags = other_tags;

        let mut platform_tags = self
            .platform_tags
            .iter()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        platform_tags.sort();
        let platform_tags = platform_tags;

        Ok(Tags::new(platform_tags, other_tags))
    }

    async fn list_packages(
        &self,
        source_filter: SourceFilter,
        tag_filter: &TagFilter,
        options: &ListPackagesOptions,
    ) -> Result<Vec<PackageInfo>> {
        Ok(self
            .filter_packages(
                self.read_packages(options.show_progress).await?,
                source_filter,
                None,
                tag_filter,
            )?
            .into_iter()
            .map(Self::make_package_info)
            .collect::<Vec<_>>())
    }

    async fn get_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &GetPackageOptions,
    ) -> Result<Option<PackageInfo>> {
        let version = downcast_version!(version);
        let package = self
            .read_package(version, tag_filter, options.show_progress)
            .await?;
        Ok(package.map(Self::make_package_info))
    }

    async fn download_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &DownloadPackageOptions,
    ) -> Result<()> {
        let version = downcast_version!(version);
        let Some(info) = self
            .read_package(version, tag_filter, options.show_progress)
            .await?
        else {
            bail!(
                "No package with ID {moniker}:{version} and tags {tags:?} found in index",
                moniker = self.moniker,
                tags = tag_filter.tags
            );
        };

        let checksum = get_checksum(&self.ctx, &info.package, options.show_progress).await?;
        let options = DownloadAssetOptionsBuilder::default()
            .update(false)
            .checksum(Some(checksum))
            .show_progress(options.show_progress)
            .build()?;
        _ = self.ctx.download_asset(&info.package.url, &options).await?;
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
        let Some(info) = self
            .read_package(version, tag_filter, options.show_progress)
            .await?
        else {
            bail!(
                "No package with ID {moniker}:{version} and tags {tags:?} found in index",
                moniker = self.moniker,
                tags = tag_filter.tags
            );
        };

        let Some(path) = &info.path else {
            bail!(
                "Package with ID {moniker}:{version} and tags {tags:?} not downloaded: use \"isopy download <PACKAGE-ID>\" or pass \"--download\" to download missing packages",
                moniker = self.moniker,
                tags = tag_filter.tags
            );
        };

        info.package
            .metadata
            .archive_type
            .unpack(path, dir, options)
            .await?;

        Self::on_after_install(dir)?;

        Ok(Package::new(info.package))
    }
}
