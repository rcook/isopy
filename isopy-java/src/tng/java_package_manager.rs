use anyhow::Result;
use async_trait::async_trait;
use isopy_lib::tng::{Context, PackageManagerOps, PackageVersion};

pub(crate) struct JavaPackageManager {}

impl JavaPackageManager {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl PackageManagerOps for JavaPackageManager {
    async fn list_categories(&self, _ctx: &dyn Context) -> Result<()> {
        todo!()
    }
    async fn list_packages(&self, _ctx: &dyn Context) -> Result<()> {
        todo!()
    }

    async fn download_package(&self, _ctx: &dyn Context, _version: &PackageVersion) -> Result<()> {
        todo!()
    }
}
