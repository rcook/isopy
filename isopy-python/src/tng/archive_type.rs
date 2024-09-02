use anyhow::Result;
use decompress::{decompress, ExtractOptsBuilder};
use std::path::Path;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub(crate) enum ArchiveType {
    TarGz,
    TarZst,
    Zip,
}

impl ArchiveType {
    pub(crate) fn suffix(&self) -> &str {
        match self {
            Self::TarGz => ".tar.gz",
            Self::TarZst => ".tar.zst",
            Self::Zip => ".zip",
        }
    }

    pub(crate) async fn unpack(&self, archive_path: &Path, dir: &Path) -> Result<()> {
        decompress(archive_path, dir, &ExtractOptsBuilder::default().build()?)?;
        Ok(())
    }
}
