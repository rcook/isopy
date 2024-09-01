use crate::python_package_manager::PythonPackageManager;
use anyhow::Result;
use async_trait::async_trait;
use isopy_lib::{Context, PackageManager, PackageManagerFactory, PackageManagerFactoryOps};

pub(crate) struct PythonPackageManagerFactory;

impl PythonPackageManagerFactory {
    pub(crate) async fn new() -> Result<PackageManagerFactory> {
        Ok(Box::new(Self))
    }
}

#[async_trait]
impl PackageManagerFactoryOps for PythonPackageManagerFactory {
    async fn make_package_manager(&self, ctx: &dyn Context) -> Result<PackageManager> {
        let package_manager = PythonPackageManager::new(ctx).await?;
        Ok(Box::new(package_manager))
    }
}
