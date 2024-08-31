use crate::app_context::AppContext;
use anyhow::{anyhow, Result};
use isopy_api::{PackageManagerFactory, PackageVersion};
use isopy_java::get_package_manager_factory as get_package_manager_factory_java;
use isopy_python::get_package_manager_factory as get_package_manager_factory_python;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct App {
    #[allow(unused)]
    config_dir: PathBuf,
    cache_dir: PathBuf,
    #[allow(unused)]
    package_manager_factories: Vec<&'static PackageManagerFactory>,
    package_manager_factory_map: HashMap<&'static str, &'static PackageManagerFactory>,
}

impl App {
    pub fn new<P>(config_dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let config_dir = config_dir.into();

        let cache_dir = config_dir.join("cache");

        let package_manager_factories = vec![
            get_package_manager_factory_java(),
            get_package_manager_factory_python(),
        ];

        let package_manager_factory_map = package_manager_factories
            .iter()
            .map(|f| (f.name(), *f))
            .collect::<HashMap<_, _>>();

        Self {
            config_dir,
            cache_dir,
            package_manager_factories,
            package_manager_factory_map,
        }
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn download_package(&self, name: &str, version: &PackageVersion) -> Result<()> {
        let package_manager_factory = self.get_package_manager_factory(name)?;
        let package_manager = package_manager_factory.make(package_manager_factory.name())?;
        let ctx = AppContext::new(self, package_manager.name());
        package_manager.download_package(&ctx, version)?;
        Ok(())
    }

    /*
    pub fn package_manager_factories(&self) -> Iter<'_, &'static PackageManagerFactory> {
        self.package_manager_factories.iter()
    }
    */

    fn get_package_manager_factory<S>(&self, name: S) -> Result<&PackageManagerFactory>
    where
        S: AsRef<str>,
    {
        let package_manager_factory = *self
            .package_manager_factory_map
            .get(name.as_ref())
            .ok_or_else(|| anyhow!("No package manager factory with name {}", name.as_ref()))?;
        Ok(package_manager_factory)
    }
}
