use crate::tng::app_context::AppContext;
use crate::tng::app_package_manager::AppPackageManager;
use anyhow::{anyhow, Result};
use isopy_java::get_package_manager_factory as get_package_manager_factory_java;
use isopy_lib::tng::PackageManagerFactory;
use isopy_python::get_package_manager_factory as get_package_manager_factory_python;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) struct App {
    cache_dir: PathBuf,
    package_manager_factories: HashMap<&'static str, PackageManagerFactory>,
}

impl App {
    pub(crate) async fn new(config_dir: &Path) -> Result<Self> {
        let cache_dir = config_dir.join("cache");
        let package_manager_factories = HashMap::from([
            ("java", get_package_manager_factory_java().await?),
            ("python", get_package_manager_factory_python().await?),
        ]);
        Ok(Self {
            cache_dir,
            package_manager_factories,
        })
    }

    pub(crate) async fn get_package_manager(&self, name: &str) -> Result<AppPackageManager> {
        let package_manager_factory = self
            .package_manager_factories
            .get(name)
            .ok_or_else(|| anyhow!("No package manager factory with name {name}"))?;
        let cache_dir = self.cache_dir.join(name);
        let ctx = AppContext::new(cache_dir);
        let package_manager = package_manager_factory.make_package_manager(&ctx).await?;
        Ok(AppPackageManager::new(ctx, package_manager))
    }
}
