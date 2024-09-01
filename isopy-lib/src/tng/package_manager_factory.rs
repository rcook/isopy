use crate::tng::context::Context;
use crate::tng::package_manager::PackageManager;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PackageManagerFactoryOps: Sync {
    async fn make_package_manager(&self, ctx: &dyn Context) -> Result<PackageManager>;
}

pub type PackageManagerFactory = Box<dyn PackageManagerFactoryOps>;
