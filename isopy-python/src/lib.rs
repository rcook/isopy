mod python_package_manager;

use crate::python_package_manager::PythonPackageManager;
use isopy_api::{PackageManager, PackageManagerFactory};
use std::sync::LazyLock;

static PYTHON_PACKAGE_FACTORY: LazyLock<PackageManagerFactory> = LazyLock::new(|| {
    fn make_package_factory<S>(name: S) -> Box<dyn PackageManager>
    where
        S: Into<String>,
    {
        Box::new(PythonPackageManager::new(name))
    }

    PackageManagerFactory::new("python", make_package_factory)
});

pub fn get_package_manager_factory() -> &'static PackageManagerFactory {
    &*PYTHON_PACKAGE_FACTORY
}
