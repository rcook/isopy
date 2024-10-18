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
use crate::constants::DEFAULT_TAGS;
use crate::index::Index;
use crate::python_package::PythonPackage;
use crate::python_package_info::PythonPackageInfo;
use anyhow::{bail, Result};
use async_trait::async_trait;
use isopy_lib::{
    DownloadFileOptionsBuilder, DownloadPackageOptions, GetPackageOptions, InstallPackageError,
    InstallPackageOptions, ListPackagesOptions, ListTagsOptions, Package, PackageAvailability,
    PackageInfo, PackageManagerContext, PackageManagerOps, SourceFilter, TagFilter, Tags,
    UpdateIndexOptions, Version,
};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::Path;
use std::result::Result as StdResult;
use tokio::fs::read_to_string;
use url::Url;

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
}

impl PythonPackageManager {
    pub(crate) fn new(ctx: PackageManagerContext, moniker: &str, url: &Url) -> Self {
        Self {
            ctx,
            moniker: String::from(moniker),
            url: url.clone(),
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

    async fn get_index(&self, update: bool, show_progress: bool) -> Result<Index> {
        let options = DownloadFileOptionsBuilder::json()
            .update(update)
            .show_progress(show_progress)
            .build()?;
        let path = self.ctx.download_file(&self.url, &options).await?;
        read_to_string(path).await?.parse()
    }
}

#[async_trait]
impl PackageManagerOps for PythonPackageManager {
    async fn update_index(&self, options: &UpdateIndexOptions) -> Result<()> {
        self.get_index(true, options.show_progress).await?;
        Ok(())
    }

    async fn list_tags(&self, options: &ListTagsOptions) -> Result<Tags> {
        let mut tags = HashSet::new();
        let mut other_tags = HashSet::new();
        let index = self.get_index(false, options.show_progress).await?;
        for item in index.items() {
            for package in PythonPackage::parse_multi(&item)? {
                tags.extend(package.metadata().tags().to_owned());
                other_tags.insert(String::from(
                    package.metadata().index_version().release_group().as_str(),
                ));
            }
        }

        let mut tags = tags.into_iter().collect::<Vec<_>>();
        tags.sort();
        let tags = tags;

        let mut other_tags = other_tags.into_iter().collect::<Vec<_>>();
        other_tags.sort();
        let other_tags = other_tags;

        let mut default_tags = DEFAULT_TAGS
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        default_tags.sort();
        let default_tags = default_tags;

        Ok(Tags::new(tags, default_tags, other_tags))
    }

    async fn list_packages(
        &self,
        sources: SourceFilter,
        tag_filter: &TagFilter,
        options: &ListPackagesOptions,
    ) -> Result<Vec<PackageInfo>> {
        use isopy_lib::SourceFilter::*;
        let index = self.get_index(false, options.show_progress).await?;
        let packages = PythonPackageInfo::read_multi(&self.ctx, &index).await?;
        let tags = tag_filter.tags(&DEFAULT_TAGS);
        let mut packages = packages
            .into_iter()
            .filter(|p| {
                matches!(
                    (sources, p.availability == PackageAvailability::Local),
                    (All, _) | (Local, true) | (Remote, false)
                )
            })
            .filter(|p| p.details.metadata().has_tags(&tags))
            .collect::<Vec<_>>();

        packages.sort_by(|a, b| {
            let temp = b.availability.cmp(&a.availability);
            if temp != Ordering::Equal {
                return temp;
            }
            b.details
                .metadata()
                .index_version()
                .cmp(a.details.metadata().index_version())
        });

        Ok(packages
            .into_iter()
            .map(PythonPackageInfo::into_package_info)
            .collect())
    }

    async fn get_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &GetPackageOptions,
    ) -> Result<Option<PackageInfo>> {
        let version = downcast_version!(version);
        let index = self.get_index(false, options.show_progress).await?;
        let tags = tag_filter.tags(&DEFAULT_TAGS);
        let package = PythonPackageInfo::read(&self.ctx, &index, version, &tags).await?;
        Ok(package.map(PythonPackageInfo::into_package_info))
    }

    async fn download_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &DownloadPackageOptions,
    ) -> Result<()> {
        let version = downcast_version!(version);
        let index = self.get_index(false, options.show_progress).await?;
        let tags = tag_filter.tags(&DEFAULT_TAGS);
        let Some(package) = PythonPackageInfo::read(&self.ctx, &index, version, &tags).await?
        else {
            bail!(
                "No package with ID {moniker}:{version} and tags {tags:?} found",
                moniker = self.moniker
            );
        };

        let checksum = get_checksum(&package.details)?;
        let options = DownloadFileOptionsBuilder::default()
            .update(false)
            .checksum(Some(checksum))
            .show_progress(options.show_progress)
            .build()?;
        _ = self
            .ctx
            .download_file(package.details.url(), &options)
            .await?;
        Ok(())
    }

    async fn install_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        dir: &Path,
        options: &InstallPackageOptions,
    ) -> StdResult<Package, InstallPackageError> {
        let version = downcast_version!(version);
        let index = self.get_index(false, options.show_progress).await?;
        let tags = tag_filter.tags(&DEFAULT_TAGS);
        let Some(package) = PythonPackageInfo::read(&self.ctx, &index, version, &tags).await?
        else {
            log::error!(
                "No package with ID {moniker}:{version} and tags {tags:?} found",
                moniker = self.moniker
            );
            return Err(InstallPackageError::VersionNotFound);
        };

        let Some(package_path) = package.path else {
            return Err(InstallPackageError::PackageNotDownloaded);
        };

        package
            .details
            .metadata()
            .archive_type()
            .unpack(&package_path, dir, options)
            .await?;

        Self::on_after_install(dir)?;

        Ok(Package::new(package.details))
    }
}
