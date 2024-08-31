use crate::archive_info::ArchiveInfo;
use crate::archive_metadata::ArchiveMetadata;
use anyhow::{bail, Result};
use isopy_api::{Accept, Context, PackageManager, Url};
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

        let index = download_json(ctx, &INDEX_URL)?;
        let items = g!(index.as_array());
        let mut all_tags = HashSet::new();
        for item in items {
            //let name = g!(g!(item.get("tag_name")).as_str());
            for archive in get_archives(item)? {
                println!("{}", archive.metadata().name());
                all_tags.extend(archive.metadata().tags().to_owned());
            }
        }

        println!("all_tags={:?}", all_tags);
        Ok(())
    }
}
