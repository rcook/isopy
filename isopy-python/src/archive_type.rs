use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum ArchiveType {
    TarGz,
    TarZst,
}

impl ArchiveType {
    pub fn suffix(&self) -> &str {
        match self {
            Self::TarGz => ".tar.gz",
            Self::TarZst => ".tar.zst",
        }
    }
}
