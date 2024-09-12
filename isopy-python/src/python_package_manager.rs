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
use crate::archive_info::ArchiveInfo;
use crate::archive_metadata::ArchiveMetadata;
use crate::checksum::get_checksum;
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use isopy_lib::tng::{
    DownloadOptions, OptionalTags, PackageFilter, PackageInfo, PackageKind, PackageManagerContext,
    PackageManagerOps, PackageSummary, Tags, Version, VersionTriple,
};
use serde_json::Value;
use std::collections::HashSet;
use std::iter::once;
use std::path::Path;
use tokio::fs::read_to_string;
use url::Url;

macro_rules! g {
    ($e : expr) => {
        match $e {
            Some(value) => value,
            None => bail!("Invalid index"),
        }
    };
}

macro_rules! downcast_version {
    ($version : expr) => {
        $version
            .as_any()
            .downcast_ref::<VersionTriple>()
            .ok_or_else(|| anyhow!("Invalid version type"))?
    };
}

pub(crate) struct PythonPackageManager {
    ctx: PackageManagerContext,
    url: Url,
}

impl PythonPackageManager {
    pub(crate) fn new(ctx: PackageManagerContext, url: &Url) -> Self {
        Self {
            ctx,
            url: url.clone(),
        }
    }

    fn get_archives(item: &Value) -> Result<Vec<ArchiveInfo>> {
        fn filter_fn(name: &str) -> bool {
            name.starts_with("cpython-") && !name.ends_with(".sha256") && name != "SHA256SUMS"
        }

        let assets = g!(g!(item.get("assets")).as_array());
        let assets = assets
            .into_iter()
            .map(|asset| {
                let url = g!(g!(asset.get("browser_download_url")).as_str()).parse::<Url>()?;
                let name = g!(g!(asset.get("name")).as_str());
                Ok((url, name))
            })
            .collect::<Result<Vec<_>>>()?;
        let archives = assets
            .into_iter()
            .filter(|(_, name)| filter_fn(*name))
            .map(|(url, name)| {
                let metadata = name.parse::<ArchiveMetadata>()?;
                let archive_info = ArchiveInfo::new(&url, metadata);
                Ok(archive_info)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(archives)
    }

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    fn get_default_tags() -> HashSet<&'static str> {
        HashSet::from(["aarch64", "unknown", "linux", "gnu", "install_only"])
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    fn get_default_tags() -> HashSet<&'static str> {
        HashSet::from(["x86_64", "unknown", "linux", "gnu", "install_only"])
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn get_default_tags() -> HashSet<&'static str> {
        HashSet::from(["aarch64", "apple", "darwin", "install_only"])
    }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    fn get_default_tags() -> HashSet<&'static str> {
        HashSet::from(["x86_64", "apple", "darwin", "install_only"])
    }

    #[cfg(target_os = "windows")]
    fn get_default_tags() -> HashSet<&'static str> {
        HashSet::from(["x86_64", "pc", "windows", "msvc", "shared", "install_only"])
    }

    fn get_tags(tags: &OptionalTags) -> HashSet<&str> {
        tags.as_ref()
            .map(|t| t.iter().map(|item| item.as_str()).collect::<HashSet<_>>())
            .unwrap_or_else(|| Self::get_default_tags())
    }

    fn get_archive(
        index: &Value,
        version: &VersionTriple,
        tags: &OptionalTags,
    ) -> Result<ArchiveInfo> {
        let tags = Self::get_tags(tags);
        let mut archives = Vec::new();
        for item in g!(index.as_array()) {
            archives.extend(Self::get_archives(item)?.into_iter().filter(|archive| {
                let m = archive.metadata();
                Self::metadata_has_tags(m, &tags) && m.full_version().version() == version
            }));
        }

        if archives.is_empty() {
            bail!("No matching archives found")
        }

        archives.sort_by_cached_key(|archive| archive.metadata().full_version().clone());
        archives.reverse();
        Ok(archives
            .into_iter()
            .next()
            .expect("Vector must contain at least one element"))
    }

    fn metadata_has_tags(metadata: &ArchiveMetadata, tags: &HashSet<&str>) -> bool {
        metadata
            .tags()
            .iter()
            .map(|t| t.as_str())
            .chain(once(metadata.full_version().build_tag().as_str()))
            .collect::<HashSet<_>>()
            .is_superset(tags)
    }

    async fn get_index(&self, update: bool) -> Result<Value> {
        let options = DownloadOptions::json().update(update);
        let path = self.ctx.download_file(&self.url, &options).await?;
        let s = read_to_string(path).await?;
        let index = serde_json::from_str(&s)?;
        Ok(index)
    }
}

#[async_trait]
impl PackageManagerOps for PythonPackageManager {
    async fn update_index(&self) -> Result<()> {
        self.get_index(true).await?;
        Ok(())
    }

    async fn list_tags(&self) -> Result<Tags> {
        let mut tags = HashSet::new();
        let mut other_tags = HashSet::new();
        let index = self.get_index(false).await?;
        for item in g!(index.as_array()) {
            for archive in Self::get_archives(item)? {
                tags.extend(archive.metadata().tags().to_owned());
                other_tags.insert(String::from(
                    archive.metadata().full_version().build_tag().as_str(),
                ));
            }
        }

        let mut tags = Vec::from_iter(tags.into_iter());
        tags.sort();
        let tags = tags;

        let mut other_tags = Vec::from_iter(other_tags.into_iter());
        other_tags.sort();
        let other_tags = other_tags;

        let mut default_tags = Self::get_default_tags()
            .into_iter()
            .map(|t| String::from(t))
            .collect::<Vec<_>>();
        default_tags.sort();
        let default_tags = default_tags;

        Ok(Tags::new(tags, default_tags, other_tags))
    }

    async fn list_packages(
        &self,
        filter: PackageFilter,
        tags: &OptionalTags,
    ) -> Result<Vec<PackageSummary>> {
        let tags = Self::get_tags(tags);
        let mut records = Vec::new();
        let index = self.get_index(false).await?;
        for item in g!(index.as_array()) {
            for archive in Self::get_archives(item)? {
                if Self::metadata_has_tags(archive.metadata(), &tags) {
                    let (kind, path) = match self.ctx.get_file(archive.url()).await {
                        Ok(p) => (PackageKind::Local, Some(p)),
                        _ => (PackageKind::Remote, None),
                    };
                    let is_local = kind == PackageKind::Local;
                    match filter {
                        PackageFilter::All => records.push((kind, archive, path)),
                        PackageFilter::Local if is_local => records.push((kind, archive, path)),
                        PackageFilter::Remote if !is_local => records.push((kind, archive, path)),
                        _ => {}
                    }
                }
            }
        }

        records.sort_by(|a, b| {
            if a.0 == b.0 {
                b.1.metadata()
                    .full_version()
                    .cmp(a.1.metadata().full_version())
            } else {
                b.0.cmp(&a.0)
            }
        });

        Ok(records
            .into_iter()
            .map(|(kind, archive, path)| {
                PackageSummary::new(
                    kind,
                    archive.metadata().name(),
                    archive.url(),
                    Version::new(archive.metadata().full_version().version().clone()),
                    path,
                )
            })
            .collect())
    }

    async fn download_package(&self, version: &Version, tags: &OptionalTags) -> Result<()> {
        let version = downcast_version!(version);
        let index = self.get_index(false).await?;
        let archive = Self::get_archive(&index, version, tags)?;
        let checksum = get_checksum(&archive)?;
        let options = DownloadOptions::default().checksum(Some(checksum));
        _ = self.ctx.download_file(archive.url(), &options).await?;
        Ok(())
    }

    async fn install_package(
        &self,
        version: &Version,
        tags: &OptionalTags,
        dir: &Path,
    ) -> Result<PackageInfo> {
        let version = downcast_version!(version);
        let index = self.get_index(false).await?;
        let archive = Self::get_archive(&index, version, tags)?;
        let archive_path = self.ctx.get_file(archive.url()).await?;
        archive
            .metadata()
            .archive_type()
            .unpack(&archive_path, dir)
            .await?;
        Ok(PackageInfo::new(archive))
    }

    async fn on_before_install(&self, _output_dir: &Path, _bin_subdir: &Path) -> Result<()> {
        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    async fn on_after_install(&self, output_dir: &Path, bin_subdir: &Path) -> Result<()> {
        use log::trace;
        use std::os::unix::fs::symlink;

        let bin_dir = output_dir.join(bin_subdir).join("bin");
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
    async fn on_after_install(&self, output_dir: &Path, bin_subdir: &Path) -> Result<()> {
        use log::trace;
        use std::fs::write;

        let cmd_path = output_dir.join(bin_subdir).join("python3.cmd");
        if !cmd_path.exists() {
            const WRAPPER: &str = "@echo off\n\"%~dp0python.exe\" %*\n";
            trace!("Creating wrapper script {}", cmd_path.display());
            write(&cmd_path, WRAPPER)?;
            trace!("Created wrapper script {}", cmd_path.display());
        }
        Ok(())
    }
}
