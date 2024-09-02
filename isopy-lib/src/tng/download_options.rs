use crate::tng::accept::Accept;
use crate::tng::checksum::Checksum;

pub struct DownloadOptions {
    pub accept: Option<Accept>,
    pub update: bool,
    pub checksum: Option<Checksum>,
}

impl DownloadOptions {
    pub fn json() -> Self {
        Self::default().accept(Some(Accept::ApplicationJson))
    }

    pub fn accept(mut self, value: Option<Accept>) -> Self {
        self.accept = value;
        self
    }

    pub fn update(mut self, value: bool) -> Self {
        self.update = value;
        self
    }

    pub fn checksum(mut self, value: Option<Checksum>) -> Self {
        self.checksum = value;
        self
    }
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            accept: None,
            update: false,
            checksum: None,
        }
    }
}
