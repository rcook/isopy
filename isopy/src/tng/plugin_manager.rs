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
use crate::tng::consts::CACHE_DIR_NAME;
use crate::tng::package_manager_helper::PackageManagerHelper;
use crate::tng::Moniker;
use isopy_lib::tng::{PackageManager, Plugin};
use std::path::Path;

pub(crate) struct PluginManager {
    go_plugin: Plugin,
    java_plugin: Plugin,
    python_plugin: Plugin,
}

impl PluginManager {
    pub(crate) fn new() -> Self {
        Self {
            go_plugin: isopy_go::tng::new_plugin(),
            java_plugin: isopy_java::tng::new_plugin(),
            python_plugin: isopy_python::tng::new_plugin(),
        }
    }

    pub(crate) fn get_plugin(&self, moniker: &Moniker) -> &Plugin {
        match moniker {
            Moniker::Go => &self.go_plugin,
            Moniker::Java => &self.java_plugin,
            Moniker::Python => &self.python_plugin,
        }
    }

    pub(crate) fn new_package_manager(
        &self,
        moniker: &Moniker,
        config_dir: &Path,
    ) -> PackageManager {
        let cache_dir = config_dir.join(CACHE_DIR_NAME).join(moniker.dir());
        let ctx = PackageManagerHelper::new(&cache_dir);
        let plugin = self.get_plugin(moniker);
        plugin.new_package_manager(ctx)
    }
}
