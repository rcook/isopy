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
use crate::tng::consts::{
    CACHE_DIR_NAME, GO_PLUGIN_MONIKER, JAVA_PLUGIN_MONIKER, PYTHON_PLUGIN_MONIKER,
};
use anyhow::{anyhow, Result};
use isopy_lib::tng::{Context, Plugin};
use std::path::Path;
use std::sync::{Arc, Weak};

type PluginInfo = (&'static str, Plugin);

type PluginInfos = Vec<PluginInfo>;

pub(crate) struct App {
    plugins: PluginInfos,
}

impl App {
    pub(crate) fn new(config_dir: &Path) -> Arc<Self> {
        Arc::new_cyclic(|me| {
            fn make_plugin(
                me: &Weak<App>,
                moniker: &'static str,
                cache_dir: &Path,
                make: fn(Context) -> Plugin,
            ) -> PluginInfo {
                (
                    moniker,
                    make(AppContext::new(Weak::clone(&me), moniker, cache_dir)),
                )
            }

            let cache_dir = config_dir.join(CACHE_DIR_NAME);
            let plugins = Vec::from([
                make_plugin(me, GO_PLUGIN_MONIKER, &cache_dir, isopy_go::tng::new_plugin),
                make_plugin(
                    me,
                    JAVA_PLUGIN_MONIKER,
                    &cache_dir,
                    isopy_java::tng::new_plugin,
                ),
                make_plugin(
                    me,
                    PYTHON_PLUGIN_MONIKER,
                    &cache_dir,
                    isopy_python::tng::new_plugin,
                ),
            ]);
            Self { plugins }
        })
    }

    pub(crate) fn get_plugin_monikers(&self) -> Vec<String> {
        self.plugins.iter().map(|(m, _)| String::from(*m)).collect()
    }

    pub(crate) fn get_plugin(&self, moniker: &str) -> Result<&Plugin> {
        let (_, plugin) = self
            .plugins
            .iter()
            .find(|(m, _)| *m == moniker)
            .ok_or_else(|| anyhow!("No plugin with moniker {moniker}"))?;
        Ok(plugin)
    }
}
