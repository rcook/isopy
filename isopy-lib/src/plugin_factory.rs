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
use crate::plugin::Plugin;
use crate::{Descriptor, EnvInfo, IsopyLibResult};
use reqwest::Url;
use serde_json::Value;
use std::ffi::OsString;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub enum Platform {
    Unix,
    Windows,
}

#[derive(Clone, Copy, Debug)]
pub enum Shell {
    Bash,
    Cmd,
}

pub trait PluginFactory: Send + Sync {
    fn name(&self) -> &str;
    fn source_url(&self) -> &Url;
    fn read_project_config(&self, props: &Value) -> IsopyLibResult<Box<dyn Descriptor>>;
    fn parse_descriptor(&self, s: &str) -> IsopyLibResult<Box<dyn Descriptor>>;
    fn make_env_info(
        &self,
        data_dir: &Path,
        props: &Value,
        base_dir: Option<&Path>,
    ) -> IsopyLibResult<EnvInfo>;
    fn make_script_command(
        &self,
        script_path: &Path,
        platform: Platform,
        shell: Shell,
    ) -> IsopyLibResult<Option<OsString>>;
    fn make_plugin(&self, offline: bool, dir: &Path) -> Box<dyn Plugin>;
}
