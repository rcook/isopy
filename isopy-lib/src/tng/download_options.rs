use crate::tng::accept::Accept;

pub struct DownloadOptions {
    pub accept: Option<Accept>,
}

impl DownloadOptions {
    pub fn json() -> Self {
        Self::default().accept(Some(Accept::ApplicationJson))
    }

    pub fn accept(mut self, value: Option<Accept>) -> Self {
        self.accept = value;
        self
    }
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self { accept: None }
    }
}
