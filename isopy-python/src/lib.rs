use isopy_api::PackageManagerFactory;
use std::sync::LazyLock;

static PYTHON_PACKAGE_FACTORY: LazyLock<PackageManagerFactory> =
    LazyLock::new(|| PackageManagerFactory::new("python"));

pub fn get_package_factory() -> &'static PackageManagerFactory {
    &*PYTHON_PACKAGE_FACTORY
}
