use anyhow::Result;
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
        match self {
            Self::TarGz => crate::tng::tar_gz_archive::unpack(archive_path, dir).await?,
            Self::TarZst => crate::tng::tar_zst_archive::unpack(archive_path, dir).await?,
            Self::Zip => crate::tng::zip_archive::unpack(archive_path, dir).await?,
        }
        Ok(())
    }
}
