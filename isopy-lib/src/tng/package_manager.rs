use crate::tng::context::Context;
use crate::tng::package_version::PackageVersion;
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait PackageManagerOps: Send + Sync {
    async fn list_categories(&self, ctx: &dyn Context) -> Result<()>;
    async fn list_packages(&self, ctx: &dyn Context) -> Result<()>;
    async fn download_package(&self, ctx: &dyn Context, version: &PackageVersion) -> Result<()>;
    async fn install_package(
        &self,
        ctx: &dyn Context,
        version: &PackageVersion,
        dir: &Path,
    ) -> Result<()>;
}

pub type PackageManager = Box<dyn PackageManagerOps>;
