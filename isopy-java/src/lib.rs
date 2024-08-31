mod java_package_manager;

use crate::java_package_manager::JavaPackageManager;
use anyhow::Result;
use isopy_api::{PackageManager, PackageManagerFactory};
use std::sync::LazyLock;

static JAVA_PACKAGE_FACTORY: LazyLock<PackageManagerFactory> = LazyLock::new(|| {
    fn make_package_factory<S>(name: S) -> Result<Box<dyn PackageManager>>
    where
        S: Into<String>,
    {
        Ok(Box::new(JavaPackageManager::new(name)))
    }

    PackageManagerFactory::new("java", make_package_factory)
});

pub fn get_package_manager_factory() -> &'static PackageManagerFactory {
    &*JAVA_PACKAGE_FACTORY
}
