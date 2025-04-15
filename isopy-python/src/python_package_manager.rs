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
use crate::package_cache::{read_package_cache, write_package_cache};
use crate::python_package::PythonPackage;
use crate::python_package_state::PythonPackageState;
use anyhow::{bail, Result};
use async_trait::async_trait;
use isopy_lib::{
    DownloadFileOptionsBuilder, DownloadPackageOptions, GetPackageStateOptions,
    InstallPackageOptions, ListPackageStatesOptions, ListTagsOptions, Package, PackageAvailability,
    PackageManagerContext, PackageManagerOps, PackageState, SourceFilter, TagFilter, Tags,
    UpdateIndexOptions, Version,
};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::metadata;
use std::path::Path;
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

    async fn fetch_package_states(
        &self,
        cache_path: &Path,
        sources: SourceFilter,
        tag_filter: &TagFilter,
        options: &ListPackageStatesOptions,
    ) -> Result<Vec<PackageState>> {
        use isopy_lib::SourceFilter::*;

        let index = self.get_index(false, options.show_progress).await?;
        let tags = tag_filter.tags(&DEFAULT_TAGS);
        let mut states = PythonPackageState::read_all(&self.ctx, &index)
            .await?
            .into_iter()
            .filter(|p| {
                matches!(
                    (sources, p.availability() == PackageAvailability::Local),
                    (All, _) | (Local, true) | (Remote, false)
                )
            })
            .filter(|p| p.package().metadata().has_tags(&tags))
            .collect::<Vec<_>>();

        states.sort_by(|a, b| {
            let temp = b.availability().cmp(&a.availability());
            if temp != Ordering::Equal {
                return temp;
            }
            b.package()
                .metadata()
                .version()
                .cmp(a.package().metadata().version())
        });

        let packages = states
            .into_iter()
            .map(PythonPackageState::into_package_state)
            .collect::<Vec<_>>();

        write_package_cache(&cache_path, &packages)?;

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
        let index = self.get_index(false, options.show_progress).await?;
        for item in index.items() {
            for package in PythonPackage::parse_all(&item)? {
                tags.extend(package.metadata().tags().to_owned());
                if let Some(release_group) = package.metadata().version().release_group() {
                    other_tags.insert(String::from(release_group.as_str()));
                }
            }
        }

        let mut tags = tags.into_iter().collect::<Vec<_>>();
        tags.sort();
        let tags = tags;

        let mut other_tags = other_tags.into_iter().collect::<Vec<_>>();
        other_tags.sort();
        let other_tags: Vec<String> = other_tags;

        let mut default_tags = DEFAULT_TAGS
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        default_tags.sort();
        let default_tags = default_tags;

        Ok(Tags::new(tags, default_tags, other_tags))
    }

    async fn list_package_states(
        &self,
        sources: SourceFilter,
        tag_filter: &TagFilter,
        options: &ListPackageStatesOptions,
    ) -> Result<Vec<PackageState>> {
        let cache_path = self.ctx.cache_dir().join("packages.yaml");
        let index_path = self.ctx.file_exists(&self.url)?;
        let Some(index_path) = index_path else {
            return Ok(self
                .fetch_package_states(&cache_path, sources, tag_filter, options)
                .await?);
        };

        if !cache_path.exists() {
            return Ok(self
                .fetch_package_states(&cache_path, sources, tag_filter, options)
                .await?);
        }

        let index_created_at = metadata(&index_path)?.created()?;
        let cache_created_at = metadata(&cache_path)?.created()?;

        if index_created_at > cache_created_at {
            return Ok(self
                .fetch_package_states(&cache_path, sources, tag_filter, options)
                .await?);
        }

        return Ok(read_package_cache(&cache_path)?);
    }

    async fn get_package_state(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &GetPackageStateOptions,
    ) -> Result<Option<PackageState>> {
        let version = downcast_version!(version);
        let index = self.get_index(false, options.show_progress).await?;
        let tags = tag_filter.tags(&DEFAULT_TAGS);
        let state = PythonPackageState::read(&self.ctx, &index, version, &tags).await?;
        Ok(state.map(PythonPackageState::into_package_state))
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
        let Some(state) = PythonPackageState::read(&self.ctx, &index, version, &tags).await? else {
            bail!(
                "No package with ID {moniker}:{version} and tags {tags:?} found in index",
                moniker = self.moniker
            );
        };

        let checksum = get_checksum(state.package())?;
        let options = DownloadFileOptionsBuilder::default()
            .update(false)
            .checksum(Some(checksum))
            .show_progress(options.show_progress)
            .build()?;
        _ = self
            .ctx
            .download_file(state.package().url(), &options)
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
        let index = self.get_index(false, options.show_progress).await?;
        let tags = tag_filter.tags(&DEFAULT_TAGS);
        let Some(state) = PythonPackageState::read(&self.ctx, &index, version, &tags).await? else {
            bail!(
                "No package with ID {moniker}:{version} and tags {tags:?} found in index",
                moniker = self.moniker
            );
        };

        let Some(path) = state.path() else {
            bail!(
                "Package with ID {moniker}:{version} and tags {tags:?} not downloaded",
                moniker = self.moniker
            );
        };

        state
            .package()
            .metadata()
            .archive_type()
            .unpack(path, dir, options)
            .await?;

        Self::on_after_install(dir)?;

        Ok(state.into_package())
    }
}
