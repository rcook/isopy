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
use crate::tng::archive_info::ArchiveInfo;
use crate::tng::archive_metadata::ArchiveMetadata;
use crate::tng::checksum::get_checksum;
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use isopy_lib::tng::{DownloadOptions, ManagerContext, ManagerOps, Version, VersionTriple};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;
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

const INDEX_URL: LazyLock<Url> = LazyLock::new(|| {
    "https://api.github.com/repos/indygreg/python-build-standalone/releases"
        .parse()
        .expect("Invalid index URL")
});

pub(crate) struct PythonManager {
    ctx: ManagerContext,
}

impl PythonManager {
    pub(crate) fn new(ctx: ManagerContext) -> Self {
        Self { ctx }
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
    fn get_platform_keywords() -> HashSet<String> {
        HashSet::from([
            String::from("aarch64"),
            String::from("unknown"),
            String::from("linux"),
            String::from("gnu"),
            String::from("install_only"),
        ])
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    fn get_platform_keywords() -> HashSet<String> {
        HashSet::from([
            String::from("x86_64"),
            String::from("unknown"),
            String::from("linux"),
            String::from("gnu"),
            String::from("install_only"),
        ])
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn get_platform_keywords() -> HashSet<String> {
        HashSet::from([
            String::from("aarch64"),
            String::from("apple"),
            String::from("darwin"),
            String::from("install_only"),
        ])
    }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    fn get_platform_keywords() -> HashSet<String> {
        HashSet::from([
            String::from("x86_64"),
            String::from("apple"),
            String::from("darwin"),
            String::from("install_only"),
        ])
    }

    #[cfg(target_os = "windows")]
    fn get_platform_keywords() -> HashSet<String> {
        HashSet::from([
            String::from("x86_64"),
            String::from("pc"),
            String::from("windows"),
            String::from("msvc"),
            String::from("shared"),
            String::from("install_only"),
        ])
    }

    fn get_archive(index: &Value, version: &VersionTriple) -> Result<ArchiveInfo> {
        let keywords = Self::get_platform_keywords();
        let mut archives = Vec::new();
        for item in g!(index.as_array()) {
            archives.extend(Self::get_archives(item)?.into_iter().filter(|archive| {
                let m = archive.metadata();
                m.keywords().is_superset(&keywords) && m.full_version().version == *version
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

    async fn get_index(&self, update: bool) -> Result<Value> {
        let options = DownloadOptions::json().update(update);
        let path = self.ctx.download_file(&INDEX_URL, &options).await?;
        let s = read_to_string(path).await?;
        let index = serde_json::from_str(&s)?;
        Ok(index)
    }
}

#[async_trait]
impl ManagerOps for PythonManager {
    async fn update_index(&self) -> Result<()> {
        self.get_index(true).await?;
        Ok(())
    }

    async fn list_categories(&self) -> Result<()> {
        let mut groups = HashSet::new();
        let mut keywords = HashSet::new();
        let index = self.get_index(false).await?;
        for item in g!(index.as_array()) {
            groups.insert(g!(g!(item.get("tag_name")).as_str()));
            for archive in Self::get_archives(item)? {
                keywords.extend(archive.metadata().keywords().to_owned());
            }
        }

        let mut groups = Vec::from_iter(groups);
        if !groups.is_empty() {
            println!("Groups:");
            groups.sort();
            groups.reverse();
            for group in groups {
                println!("  {}", group);
            }
        }

        let mut keywords = Vec::from_iter(keywords);
        if !keywords.is_empty() {
            println!("Keywords:");
            keywords.sort();
            for keyword in keywords {
                println!("  {}", keyword)
            }
        }

        let mut platform_keywords = Vec::from_iter(Self::get_platform_keywords());
        if !platform_keywords.is_empty() {
            println!("Platform keywords:");
            platform_keywords.sort();
            for platform_keyword in platform_keywords {
                println!("  {}", platform_keyword)
            }
        }

        Ok(())
    }

    async fn list_packages(&self) -> Result<()> {
        let platform_keywords = Self::get_platform_keywords();
        let mut archives = Vec::new();
        let index = self.get_index(false).await?;
        for item in g!(index.as_array()) {
            for archive in Self::get_archives(item)? {
                if archive
                    .metadata()
                    .keywords()
                    .is_superset(&platform_keywords)
                {
                    archives.push(archive);
                }
            }
        }

        archives.sort_by_cached_key(|x| x.metadata().full_version().clone());
        archives.reverse();
        for archive in archives {
            println!("{}", archive.metadata().name());
        }
        Ok(())
    }

    async fn download_package(&self, version: &Version) -> Result<()> {
        let version = version
            .as_any()
            .downcast_ref::<VersionTriple>()
            .ok_or_else(|| anyhow!("Invalid version type"))?;
        let index = self.get_index(false).await?;
        let archive = Self::get_archive(&index, version)?;
        let checksum = get_checksum(&archive)?;
        let options = DownloadOptions::default().checksum(Some(checksum));
        _ = self.ctx.download_file(archive.url(), &options).await?;
        Ok(())
    }

    async fn install_package(&self, version: &Version, dir: &Path) -> Result<()> {
        let version = version
            .as_any()
            .downcast_ref::<VersionTriple>()
            .ok_or_else(|| anyhow!("Invalid version type"))?;
        let index = self.get_index(false).await?;
        let archive = Self::get_archive(&index, version)?;
        let archive_path = self.ctx.get_file(archive.url()).await?;
        archive
            .metadata()
            .archive_type()
            .unpack(&archive_path, dir)
            .await?;
        println!("{}", archive_path.display());
        Ok(())
    }
}
