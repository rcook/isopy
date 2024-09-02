use anyhow::Result;
use std::path::Path;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub(crate) enum ArchiveType {
    TarGz,
    TarZst,
}

impl ArchiveType {
    pub(crate) fn suffix(&self) -> &str {
        match self {
            Self::TarGz => ".tar.gz",
            Self::TarZst => ".tar.zst",
        }
    }

    pub(crate) async fn unpack(&self, archive_path: &Path, dir: &Path) -> Result<()> {
        match self {
            Self::TarGz => Self::unpack_tar_gz(archive_path, dir),
            Self::TarZst => Self::unpack_tar_zst(archive_path, dir),
        }
    }

    fn unpack_tar_gz(archive_path: &Path, dir: &Path) -> Result<()> {
        todo!(
            "Unpack .tar.gz archive {} to {}",
            archive_path.display(),
            dir.display()
        )
    }

    fn unpack_tar_zst(archive_path: &Path, dir: &Path) -> Result<()> {
        todo!(
            "Unpack .tar.zst archive {} to {}",
            archive_path.display(),
            dir.display()
        )
    }
}
