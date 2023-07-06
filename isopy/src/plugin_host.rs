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
use anyhow::Result;
use isopy_lib::{Descriptor, EnvInfo, Plugin, PluginFactory};
use std::ffi::OsStr;
use std::path::Path;
use url::Url;

pub struct PluginHost {
    prefix: String,
    plugin_factory: Box<dyn PluginFactory>,
}

impl PluginHost {
    pub fn new(prefix: &str, plugin_factory: Box<dyn PluginFactory>) -> Self {
        Self {
            prefix: String::from(prefix),
            plugin_factory,
        }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn name(&self) -> &str {
        self.plugin_factory.name()
    }

    pub fn source_url(&self) -> &Url {
        self.plugin_factory.source_url()
    }

    pub fn parse_descriptor(&self, s: &str) -> Result<Box<dyn Descriptor>> {
        Ok(self.plugin_factory.parse_descriptor(s)?)
    }

    pub fn project_config_file_name(&self) -> &OsStr {
        self.plugin_factory.project_config_file_name()
    }

    pub fn read_project_config_file(&self, path: &Path) -> Result<Box<dyn Descriptor>> {
        Ok(self.plugin_factory.read_project_config_file(path)?)
    }

    pub fn read_env_config(
        &self,
        data_dir: &Path,
        properties: &serde_json::Value,
    ) -> Result<EnvInfo> {
        Ok(self.plugin_factory.read_env_config(data_dir, properties)?)
    }

    pub fn make_plugin(&self, dir: &Path) -> Box<dyn Plugin> {
        self.plugin_factory.make_plugin(dir)
    }
}
