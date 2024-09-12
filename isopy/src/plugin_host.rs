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
use isopy_lib::tng::EnvProps;
use isopy_lib::{Descriptor, EnvInfo, IsopyLibResult, Platform, PluginFactory, Shell};
use serde_json::Value;
use std::ffi::OsString;
use std::path::Path;
use std::sync::Arc;

pub(crate) struct PluginHost {
    prefix: String,
    plugin_factory: Box<dyn PluginFactory>,
}

pub(crate) type PluginHostRef = Arc<PluginHost>;

impl PluginHost {
    pub(crate) fn new(prefix: &str, plugin_factory: Box<dyn PluginFactory>) -> Self {
        Self {
            prefix: String::from(prefix),
            plugin_factory,
        }
    }

    pub(crate) fn prefix(&self) -> &str {
        &self.prefix
    }
}

impl PluginFactory for PluginHost {
    fn read_project_config(&self, props: &Value) -> IsopyLibResult<Box<dyn Descriptor>> {
        self.plugin_factory.read_project_config(props)
    }

    fn make_env_info(
        &self,
        data_dir: &Path,
        env_props: &EnvProps,
        base_dir: Option<&Path>,
    ) -> IsopyLibResult<EnvInfo> {
        self.plugin_factory
            .make_env_info(data_dir, env_props, base_dir)
    }

    fn make_script_command(
        &self,
        script_path: &Path,
        platform: Platform,
        shell: Shell,
    ) -> IsopyLibResult<Option<OsString>> {
        self.plugin_factory
            .make_script_command(script_path, platform, shell)
    }
}
