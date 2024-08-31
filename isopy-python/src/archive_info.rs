use crate::archive_metadata::ArchiveMetadata;
use isopy_api::Url;

#[derive(Debug)]
pub struct ArchiveInfo {
    url: Url,
    metadata: ArchiveMetadata,
}

impl ArchiveInfo {
    pub fn new(url: Url, metadata: ArchiveMetadata) -> Self {
        Self { url, metadata }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn metadata(&self) -> &ArchiveMetadata {
        &self.metadata
    }
}
