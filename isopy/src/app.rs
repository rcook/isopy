use crate::app_context::AppContext;
use anyhow::{anyhow, Result};
use isopy_api::{PackageManagerFactory, PackageVersion};
use isopy_java::get_package_manager_factory as get_package_manager_factory_java;
use isopy_python::get_package_manager_factory as get_package_manager_factory_python;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct App {
    cache_dir: PathBuf,
    package_manager_factories: HashMap<&'static str, &'static PackageManagerFactory>,
}

impl App {
    pub fn new(config_dir: &Path) -> Self {
        let cache_dir = config_dir.join("cache");
        let package_manager_factories = HashMap::from([
            ("java", get_package_manager_factory_java()),
            ("python", get_package_manager_factory_python()),
        ]);
        Self {
            cache_dir,
            package_manager_factories,
        }
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn download_package(&self, name: &str, version: &PackageVersion) -> Result<()> {
        let package_manager_factory = *self
            .package_manager_factories
            .get(name)
            .ok_or_else(|| anyhow!("No package manager factory with name {name}"))?;
        let ctx = AppContext::new(self, name);
        let package_manager = package_manager_factory.make(&ctx)?;
        package_manager.download_package(&ctx, version)?;
        Ok(())
    }
}
