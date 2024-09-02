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
            Self::TarGz => todo!(
                "Unpack .tar.gz archive {} to {}",
                archive_path.display(),
                dir.display()
            ),
            Self::TarZst => todo!(
                "Unpack .tar.zst archive {} to {}",
                archive_path.display(),
                dir.display()
            ),
        }
    }
}
