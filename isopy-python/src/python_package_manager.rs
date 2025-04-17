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
use crate::constants::PLATFORM_TAGS;
use crate::index::Index;
use crate::local_package_info::LocalPackageInfo;
use crate::package_cache::{read_package_cache, write_package_cache};
use crate::platform_hacks::choose_best;
use crate::python_package::PythonPackage;
use crate::python_version::PythonVersion;
use anyhow::{bail, Result};
use async_trait::async_trait;
use isopy_lib::{
    DownloadAssetOptionsBuilder, DownloadPackageOptions, GetPackageOptions, InstallPackageOptions,
    ListPackagesOptions, ListTagsOptions, Package, PackageInfo, PackageManagerContext,
    PackageManagerOps, SourceFilter, TagFilter, Tags, UpdateIndexOptions, Version,
};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::metadata;
use std::path::{Path, PathBuf};
use tokio::fs::read_to_string;
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
            moniker: String::from(moniker),
            url: url.clone(),
            platform_tags: PLATFORM_TAGS
                .iter()
                .copied()
                .map(String::from)
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

    async fn get_index_path(&self, update: bool, show_progress: bool) -> Result<PathBuf> {
        let options = DownloadAssetOptionsBuilder::json()
            .update(update)
            .show_progress(show_progress)
            .build()?;
        self.ctx.download_asset(&self.url, &options).await
    }

    async fn fetch_packages(
        &self,
        cache_path: &Path,
        show_progress: bool,
    ) -> Result<Vec<PythonPackage>> {
        let index_path = self.get_index_path(false, show_progress).await?;
        let index = read_to_string(index_path).await?.parse::<Index>()?;
        let mut packages = Vec::new();
        for item in index.items() {
            packages.extend(PythonPackage::read_all(&item)?.into_iter().filter_map(|p| {
                if p.metadata.tags.is_superset(&self.platform_tags) {
                    Some(p)
                } else {
                    None
                }
            }));
        }

        write_package_cache(cache_path, &packages)?;
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
        use isopy_lib::SourceFilter::*;

        let tags = tag_filter
            .tags
            .iter()
            .map(String::clone)
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

        // Must sort _before_ uniquifying
        infos.sort_by(|a, b| match (b.path.is_some(), a.path.is_some()) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => b.package.metadata.version.cmp(&a.package.metadata.version),
        });

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
        self.get_index_path(true, options.show_progress).await?;
        Ok(())
    }

    async fn list_tags(&self, options: &ListTagsOptions) -> Result<Tags> {
        let mut common_tags = HashSet::new();
        let mut other_tags = HashSet::new();
        for package in self.read_packages(options.show_progress).await? {
            common_tags.extend(package.metadata.tags.clone());
            if let Some(label) = package.metadata.version.label {
                other_tags.insert(String::from(label.as_str()));
            }
        }

        common_tags.retain(|t| !self.platform_tags.contains(t));
        let mut common_tags = common_tags.into_iter().collect::<Vec<_>>();
        common_tags.sort();
        let common_tags = common_tags;

        let mut platform_tags = self.platform_tags.clone().into_iter().collect::<Vec<_>>();
        platform_tags.sort();
        let platform_tags = platform_tags;

        let mut other_tags = other_tags.into_iter().collect::<Vec<_>>();
        other_tags.sort();
        let other_tags: Vec<String> = other_tags;

        Ok(Tags::new(platform_tags, common_tags, other_tags))
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
                tags=tag_filter.tags
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
