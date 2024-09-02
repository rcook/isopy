use anyhow::Result;
use async_trait::async_trait;
use isopy_lib::tng::{Context, PackageManagerOps, PackageVersion};
use std::path::Path;

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

    async fn install_package(
        &self,
        _ctx: &dyn Context,
        _version: &PackageVersion,
        _dir: &Path,
    ) -> Result<()> {
        todo!()
    }
}
