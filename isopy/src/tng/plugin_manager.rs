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
use crate::tng::app_host::AppHost;
use crate::tng::consts::{
    CACHE_DIR_NAME, GO_PLUGIN_MONIKER, JAVA_PLUGIN_MONIKER, PYTHON_PLUGIN_MONIKER,
};
use anyhow::{anyhow, Result};
use isopy_lib::tng::{Host, PackageManager, Plugin};
use std::path::Path;
use std::sync::{Arc, Weak};

type PluginInfo = (&'static str, Plugin);

type PluginInfos = Vec<PluginInfo>;

pub(crate) struct PluginManager {
    plugins: PluginInfos,
}

impl PluginManager {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new_cyclic(|me| {
            fn make_plugin(
                me: &Weak<PluginManager>,
                moniker: &'static str,
                make: fn(Host) -> Plugin,
            ) -> PluginInfo {
                (moniker, make(AppHost::new(Weak::clone(&me), moniker)))
            }

            let plugins = Vec::from([
                make_plugin(me, GO_PLUGIN_MONIKER, isopy_go::tng::new_plugin),
                make_plugin(me, JAVA_PLUGIN_MONIKER, isopy_java::tng::new_plugin),
                make_plugin(me, PYTHON_PLUGIN_MONIKER, isopy_python::tng::new_plugin),
            ]);
            Self { plugins }
        })
    }

    pub(crate) fn get_plugin_monikers(&self) -> Vec<&str> {
        self.plugins.iter().map(|(m, _)| *m).collect()
    }

    pub(crate) fn get_plugin(&self, moniker: &str) -> Result<&Plugin> {
        let (_, plugin) = self
            .plugins
            .iter()
            .find(|(m, _)| *m == moniker)
            .ok_or_else(|| anyhow!("No plugin with moniker {moniker}"))?;
        Ok(plugin)
    }

    pub(crate) fn new_package_manager(
        &self,
        moniker: &str,
        config_dir: &Path,
    ) -> Result<PackageManager> {
        let cache_dir = config_dir.join(CACHE_DIR_NAME).join(moniker);
        Ok(self.get_plugin(moniker)?.new_package_manager(&cache_dir))
    }
}
