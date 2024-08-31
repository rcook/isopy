mod archive_full_version;
mod archive_group;
mod archive_info;
mod archive_metadata;
mod archive_type;
mod python_package_manager;

use crate::python_package_manager::PythonPackageManager;
use anyhow::Result;
use isopy_api::{Context, PackageManager, PackageManagerFactory};
use std::sync::LazyLock;

static PYTHON_PACKAGE_FACTORY: LazyLock<PackageManagerFactory> = LazyLock::new(|| {
    fn make_package_factory(ctx: &dyn Context) -> Result<Box<dyn PackageManager>> {
        let package_manager = PythonPackageManager::new(ctx)?;
        Ok(Box::new(package_manager))
    }

    PackageManagerFactory::new(make_package_factory)
});

pub fn get_package_manager_factory() -> &'static PackageManagerFactory {
    &*PYTHON_PACKAGE_FACTORY
}
