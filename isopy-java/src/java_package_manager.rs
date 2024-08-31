use anyhow::Result;
use isopy_api::{Context, PackageManager, PackageVersion};

pub struct JavaPackageManager {}

impl JavaPackageManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for JavaPackageManager {
    fn download_package(&self, _ctx: &dyn Context, _version: &PackageVersion) -> Result<()> {
        todo!()
    }
}
