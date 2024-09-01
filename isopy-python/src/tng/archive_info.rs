use crate::tng::archive_metadata::ArchiveMetadata;
use isopy_lib::tng::Url;

#[derive(Clone, Debug)]
pub(crate) struct ArchiveInfo {
    url: Url,
    metadata: ArchiveMetadata,
}

impl ArchiveInfo {
    pub(crate) fn new(url: Url, metadata: ArchiveMetadata) -> Self {
        Self { url, metadata }
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn metadata(&self) -> &ArchiveMetadata {
        &self.metadata
    }
}
