// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::tng::app_context::AppContext;
use crate::tng::app_package_manager::AppPackageManager;
use crate::tng::consts::{
    CACHE_DIR_NAME, GO_PACKAGE_MANAGER_NAME, JAVA_PACKAGE_MANAGER_NAME, PYTHON_PACKAGE_MANAGER_NAME,
};
use anyhow::{anyhow, Result};
use isopy_lib::tng::PackageManagerFactory;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) struct App {
    cache_dir: PathBuf,
    package_manager_factories: HashMap<&'static str, PackageManagerFactory>,
}

impl App {
    pub(crate) fn new(config_dir: &Path) -> Result<Self> {
        let cache_dir = config_dir.join(CACHE_DIR_NAME);
        let package_manager_factories = HashMap::from([
            (
                GO_PACKAGE_MANAGER_NAME,
                isopy_go2::make_package_manager_factory(),
            ),
            (
                JAVA_PACKAGE_MANAGER_NAME,
                isopy_java2::make_package_manager_factory(),
            ),
            (
                PYTHON_PACKAGE_MANAGER_NAME,
                isopy_python2::make_package_manager_factory(),
            ),
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
        let package_manager = package_manager_factory.make_package_manager();
        Ok(AppPackageManager::new(ctx, package_manager))
    }
}