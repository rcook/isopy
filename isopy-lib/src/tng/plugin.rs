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
use crate::env_info::EnvInfo;
use crate::tng::env_props::EnvProps;
use crate::tng::package_manager::PackageManager;
use crate::tng::package_manager_context::PackageManagerContext;
use crate::tng::version::Version;
use crate::{Platform, Shell};
use anyhow::Result;
use std::ffi::OsString;
use std::path::Path;
use url::Url;

pub trait PluginOps: Send + Sync {
    fn url(&self) -> &Url;
    fn parse_version(&self, s: &str) -> Result<Version>;
    fn make_env_info(
        &self,
        data_dir: &Path,
        env_props: &EnvProps,
        base_dir: Option<&Path>,
    ) -> EnvInfo;
    fn make_script_command(
        &self,
        script_path: &Path,
        platform: Platform,
        shell: Shell,
    ) -> Result<Option<OsString>>;
    fn new_package_manager(&self, ctx: PackageManagerContext) -> PackageManager;
}

crate::macros::dyn_trait_struct!(Plugin, PluginOps);
