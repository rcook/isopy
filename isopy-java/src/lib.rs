mod java_package_manager;

use crate::java_package_manager::JavaPackageManager;
use anyhow::Result;
use isopy_api::{Context, PackageManager, PackageManagerFactory};
use std::sync::LazyLock;

static JAVA_PACKAGE_FACTORY: LazyLock<PackageManagerFactory> = LazyLock::new(|| {
    fn make_package_factory(_ctx: &dyn Context) -> Result<Box<dyn PackageManager>> {
        Ok(Box::new(JavaPackageManager::new()))
    }

    PackageManagerFactory::new(make_package_factory)
});

pub fn get_package_manager_factory() -> &'static PackageManagerFactory {
    &*JAVA_PACKAGE_FACTORY
}
