use crate::tng::archive_info::ArchiveInfo;
use crate::tng::archive_metadata::ArchiveMetadata;
use crate::tng::checksum::get_checksum;
use anyhow::{bail, Result};
use async_trait::async_trait;
use isopy_lib::tng::{Context, DownloadOptions, PackageManagerOps, PackageVersion};
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

pub(crate) struct PythonPackageManager {
    index: Value,
}

impl PythonPackageManager {
    pub(crate) async fn new(ctx: &dyn Context) -> Result<Self> {
        let path = ctx
            .download_file(&INDEX_URL, &DownloadOptions::json())
            .await?;
        let s = read_to_string(path).await?;
        let index = serde_json::from_str(&s)?;
        Ok(Self { index })
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

    fn get_archive(index: &Value, version: &PackageVersion) -> Result<ArchiveInfo> {
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
}

#[async_trait]
impl PackageManagerOps for PythonPackageManager {
    async fn list_categories(&self, _ctx: &dyn Context) -> Result<()> {
        let mut groups = HashSet::new();
        let mut keywords = HashSet::new();
        for item in g!(self.index.as_array()) {
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

    async fn list_packages(&self, _ctx: &dyn Context) -> Result<()> {
        let platform_keywords = Self::get_platform_keywords();
        let mut archives = Vec::new();
        for item in g!(self.index.as_array()) {
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

    async fn download_package(&self, ctx: &dyn Context, version: &PackageVersion) -> Result<()> {
        let archive = Self::get_archive(&self.index, version)?;
        let checksum = get_checksum(&archive)?;
        let options = DownloadOptions::default().checksum(Some(checksum));
        _ = ctx.download_file(archive.url(), &options).await?;
        Ok(())
    }

    async fn install_package(
        &self,
        ctx: &dyn Context,
        version: &PackageVersion,
        dir: &Path,
    ) -> Result<()> {
        let archive = Self::get_archive(&self.index, version)?;
        let archive_path = ctx.get_file(archive.url()).await?;
        archive
            .metadata()
            .archive_type()
            .unpack(&archive_path, dir)
            .await?;
        println!("{}", archive_path.display());
        Ok(())
    }
}
