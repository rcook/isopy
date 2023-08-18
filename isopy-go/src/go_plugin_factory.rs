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
use crate::constants::{DOWNLOADS_URL, PLUGIN_NAME};
use crate::go_plugin::GoPlugin;
use isopy_lib::{Descriptor, EnvInfo, IsopyLibResult, Plugin, PluginFactory};
use reqwest::Url;
use serde_json::Value;
use std::ffi::OsString;
use std::path::Path;

pub struct GoPluginFactory;

impl Default for GoPluginFactory {
    fn default() -> Self {
        Self
    }
}

impl PluginFactory for GoPluginFactory {
    fn name(&self) -> &str {
        PLUGIN_NAME
    }

    fn source_url(&self) -> &Url {
        &DOWNLOADS_URL
    }

    fn read_project_config(&self, _props: &Value) -> IsopyLibResult<Box<dyn Descriptor>> {
        todo!();
    }

    fn parse_descriptor(&self, _s: &str) -> IsopyLibResult<Box<dyn Descriptor>> {
        todo!();
    }

    fn make_env_info(
        &self,
        _data_dir: &Path,
        _props: &Value,
        _base_dir: Option<&Path>,
    ) -> IsopyLibResult<EnvInfo> {
        todo!();
    }

    fn make_script_command(&self, _script_path: &Path) -> IsopyLibResult<Option<OsString>> {
        todo!();
    }

    fn make_plugin(&self, offline: bool, dir: &Path) -> Box<dyn Plugin> {
        Box::new(GoPlugin::new(offline, dir))
    }
}
