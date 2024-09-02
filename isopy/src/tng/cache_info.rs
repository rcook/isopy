use crate::tng::manifest::Manifest;
use anyhow::Result;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct CacheInfo {
    pub(crate) path: PathBuf,
    pub(crate) manifest: Manifest,
}

impl CacheInfo {
    pub(crate) fn load<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let manifest = if path.is_file() {
            let f = File::open(&path)?;
            serde_json::from_reader(f)?
        } else {
            Manifest::default()
        };
        Ok(Self { path, manifest })
    }

    pub(crate) fn save(&self) -> Result<()> {
        let f = File::create(&self.path)?;
        serde_json::to_writer_pretty(&f, &self.manifest)?;
        Ok(())
    }
}
