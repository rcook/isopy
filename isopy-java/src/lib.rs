use isopy_api::PackageManagerFactory;
use std::sync::LazyLock;

static JAVA_PACKAGE_FACTORY: LazyLock<PackageManagerFactory> =
    LazyLock::new(|| PackageManagerFactory::new("java"));

pub fn get_package_manager_factory() -> &'static PackageManagerFactory {
    &*JAVA_PACKAGE_FACTORY
}
