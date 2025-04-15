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
use crate::package_cache::{read_package_cache, write_package_cache};
use crate::python_package::PythonPackage;
use crate::python_package_with_availability::PythonPackageWithAvailability;
use anyhow::{bail, Result};
use async_trait::async_trait;
use isopy_lib::{
    DownloadAssetOptionsBuilder, DownloadPackageOptions, GetPackageOptions, InstallPackageOptions,
    ListPackagesOptions, ListTagsOptions, Package, PackageAvailability, PackageInfo,
    PackageManagerContext, PackageManagerOps, SourceFilter, TagFilter, Tags, UpdateIndexOptions,
    Version,
};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::metadata;
use std::path::Path;
use tokio::fs::read_to_string;
use url::Url;

const PACKAGE_CACHE_FILE_NAME: &str = "package-cache.yaml";

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
        let options = DownloadAssetOptionsBuilder::json()
            .update(update)
            .show_progress(show_progress)
            .build()?;
        let path = self.ctx.download_asset(&self.url, &options).await?;
        read_to_string(path).await?.parse()
    }

    async fn fetch_packages(
        &self,
        cache_path: &Path,
        show_progress: bool,
    ) -> Result<Vec<PythonPackage>> {
        let index = self.get_index(false, show_progress).await?;
        let platform_tags = PLATFORM_TAGS.iter().copied().collect::<HashSet<_>>();
        let mut packages = Vec::new();
        for item in index.items() {
            for package in PythonPackage::parse_all(&item)? {
                if package.metadata().has_tags(&platform_tags) {
                    packages.push(package);
                }
            }
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
        sources: SourceFilter,
        tag_filter: &TagFilter,
    ) -> Result<Vec<PackageInfo>> {
        use isopy_lib::SourceFilter::*;

        let mut packages = {
            let tags = tag_filter.tags(&[]);
            let mut temp_packages = Vec::new();
            for package in packages {
                if package.metadata().has_tags(&tags) {
                    let (availability, path) = match self.ctx.check_asset(package.url())? {
                        Some(p) => (PackageAvailability::Local, Some(p)),
                        None => (PackageAvailability::Remote, None),
                    };

                    if matches!(
                        (sources, availability == PackageAvailability::Local),
                        (All, _) | (Local, true) | (Remote, false)
                    ) {
                        temp_packages.push(PythonPackageWithAvailability {
                            package,
                            availability,
                            path,
                        });
                    }
                }
            }
            temp_packages
        };

        packages.sort_by(|a, b| {
            let temp = b.availability.cmp(&a.availability);
            if temp != Ordering::Equal {
                return temp;
            }
            b.package
                .metadata()
                .version()
                .cmp(a.package.metadata().version())
        });

        let packages = packages
            .into_iter()
            .map(PythonPackageWithAvailability::into_package_info)
            .collect::<Vec<_>>();
        Ok(packages)
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
        for package in self.read_packages(options.show_progress).await? {
            tags.extend(package.metadata().tags().to_owned());
            if let Some(release_group) = package.metadata().version().release_group() {
                other_tags.insert(String::from(release_group.as_str()));
            }
        }

        let mut tags = tags.into_iter().collect::<Vec<_>>();
        tags.sort();
        let tags = tags;

        let mut other_tags = other_tags.into_iter().collect::<Vec<_>>();
        other_tags.sort();
        let other_tags: Vec<String> = other_tags;

        let mut default_tags = PLATFORM_TAGS
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
        self.filter_packages(
            self.read_packages(options.show_progress).await?,
            sources,
            tag_filter,
        )
    }

    async fn get_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &GetPackageOptions,
    ) -> Result<Option<PackageInfo>> {
        let version = downcast_version!(version);

        // TBD: Use cache!
        let index = self.get_index(false, options.show_progress).await?;

        let tags = tag_filter.tags(&PLATFORM_TAGS);
        let package = PythonPackageWithAvailability::read(&self.ctx, &index, version, &tags)?;
        Ok(package.map(PythonPackageWithAvailability::into_package_info))
    }

    async fn download_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &DownloadPackageOptions,
    ) -> Result<()> {
        let version = downcast_version!(version);

        // TBD: Use cache!
        let index = self.get_index(false, options.show_progress).await?;

        let tags = tag_filter.tags(&PLATFORM_TAGS);
        let Some(package) = PythonPackageWithAvailability::read(&self.ctx, &index, version, &tags)?
        else {
            bail!(
                "No package with ID {moniker}:{version} and tags {tags:?} found in index",
                moniker = self.moniker
            );
        };

        let checksum = get_checksum(&self.ctx, &package.package, options.show_progress).await?;
        let options = DownloadAssetOptionsBuilder::default()
            .update(false)
            .checksum(Some(checksum))
            .show_progress(options.show_progress)
            .build()?;
        _ = self
            .ctx
            .download_asset(package.package.url(), &options)
            .await?;
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

        // TBD: Use cache!
        let index = self.get_index(false, options.show_progress).await?;

        let tags = tag_filter.tags(&PLATFORM_TAGS);
        let Some(package) = PythonPackageWithAvailability::read(&self.ctx, &index, version, &tags)?
        else {
            bail!(
                "No package with ID {moniker}:{version} and tags {tags:?} found in index",
                moniker = self.moniker
            );
        };

        let Some(path) = &package.path else {
            bail!(
                "Package with ID {moniker}:{version} and tags {tags:?} not downloaded",
                moniker = self.moniker
            );
        };

        package
            .package
            .metadata()
            .archive_type()
            .unpack(path, dir, options)
            .await?;

        Self::on_after_install(dir)?;

        Ok(package.into_package())
    }
}
