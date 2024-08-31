use crate::archive_full_version::ArchiveFullVersion;
use crate::archive_group::ArchiveGroup;
use crate::archive_info::ArchiveInfo;
use crate::archive_metadata::ArchiveMetadata;
use anyhow::{anyhow, bail, Result};
use isopy_api::{Accept, Context, PackageManager, PackageVersion, Url};
use serde_json::Value;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::sync::LazyLock;

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

pub struct PythonPackageManager {
    name: String,
}

impl PythonPackageManager {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl PackageManager for PythonPackageManager {
    fn name(&self) -> &str {
        &self.name
    }

    fn test(&self, ctx: &dyn Context) -> Result<()> {
        fn download_json(ctx: &dyn Context, url: &Url) -> Result<Value> {
            let path = ctx.download(url, Some(Accept::ApplicationJson))?;
            let s = read_to_string(path)?;
            let value = serde_json::from_str(&s)?;
            Ok(value)
        }

        let index = download_json(ctx, &INDEX_URL)?;
        show_summary(&index)?;
        filter_archives(&index)?;
        let archive = get_archive(
            &index,
            &PackageVersion {
                major: 3,
                minor: 12,
                revision: 5,
            },
        )?;

        let archive_path = ctx.download(archive.url(), None)?;
        println!("{archive_path:?}");
        Ok(())
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
            let archive_info = ArchiveInfo::new(url, metadata);
            Ok(archive_info)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(archives)
}

fn show_summary(index: &Value) -> Result<()> {
    let mut groups = HashSet::new();
    let mut keywords = HashSet::new();
    for item in g!(index.as_array()) {
        groups.insert(g!(g!(item.get("tag_name")).as_str()));
        for archive in get_archives(item)? {
            keywords.extend(archive.metadata().keywords().to_owned());
        }
    }

    let mut groups = groups.into_iter().collect::<Vec<_>>();
    if !groups.is_empty() {
        println!("Groups:");
        groups.sort();
        groups.reverse();
        for group in groups {
            println!("  {}", group);
        }
    }

    println!("Keywords:");
    let mut keywords = keywords.into_iter().collect::<Vec<_>>();
    if !keywords.is_empty() {
        keywords.sort();
        for keyword in keywords {
            println!("  {}", keyword)
        }
    }

    Ok(())
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

fn filter_archives(index: &Value) -> Result<()> {
    let search_keywords = get_platform_keywords();
    let mut archives = Vec::new();
    for item in g!(index.as_array()) {
        for archive in get_archives(item)? {
            if archive.metadata().keywords().is_superset(&search_keywords) {
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

fn get_archive(index: &Value, version: &PackageVersion) -> Result<ArchiveInfo> {
    fn get_groups(index: &Value) -> Result<Vec<ArchiveGroup>> {
        let mut groups = g!(index.as_array())
            .iter()
            .map(|item| g!(g!(item.get("tag_name")).as_str()).parse::<ArchiveGroup>())
            .collect::<Result<HashSet<_>>>()?
            .into_iter()
            .collect::<Vec<_>>();
        groups.sort();
        groups.reverse();
        Ok(groups)
    }

    let groups = get_groups(index)?;
    let latest_group = groups.first().ok_or_else(|| anyhow!("No groups found"))?;
    let full_version = ArchiveFullVersion {
        version: version.clone(),
        group: latest_group.clone(),
    };

    let search_keywords = get_platform_keywords();
    let mut archives = Vec::new();
    for item in g!(index.as_array()) {
        for archive in get_archives(item)? {
            if archive.metadata().keywords().is_superset(&search_keywords) {
                archives.push(archive);
            }
        }
    }

    let archives = archives
        .iter()
        .filter(|x| *x.metadata().full_version() == full_version)
        .collect::<Vec<_>>();
    match archives.len() {
        0 => bail!("No matching archive found"),
        1 => Ok((*archives
            .first()
            .expect("Vector contains exactly one element"))
        .clone()),
        _ => bail!("More than one matching archive found"),
    }
}
