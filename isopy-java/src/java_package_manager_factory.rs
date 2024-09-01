use crate::java_package_manager::JavaPackageManager;
use anyhow::Result;
use async_trait::async_trait;
use isopy_lib::{Context, PackageManager, PackageManagerFactory, PackageManagerFactoryOps};

pub(crate) struct JavaPackageManagerFactory;

impl JavaPackageManagerFactory {
    pub(crate) async fn new() -> Result<PackageManagerFactory> {
        Ok(Box::new(Self))
    }
}

#[async_trait]
impl PackageManagerFactoryOps for JavaPackageManagerFactory {
    async fn make_package_manager(&self, _ctx: &dyn Context) -> Result<PackageManager> {
        let package_manager = JavaPackageManager::new();
        Ok(Box::new(package_manager))
    }
}
