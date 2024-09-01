use crate::tng::context::Context;
use crate::tng::package_version::PackageVersion;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PackageManagerOps: Send + Sync {
    async fn download_package(&self, ctx: &dyn Context, version: &PackageVersion) -> Result<()>;
}

pub type PackageManager = Box<dyn PackageManagerOps>;
