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
use isopy_lib::{
    EnvInfo, PackageManager, PackageManagerContext, Platform, Plugin, PluginOps, Shell, Version,
};
use std::ffi::OsString;
use std::path::Path;
use std::sync::LazyLock;
use url::Url;

use crate::java_package_manager::JavaPackageManager;
use crate::java_version::JavaVersion;

static INDEX_URL: LazyLock<Url> = LazyLock::new(|| {
    "https://api.adoptium.net/"
        .parse()
        .expect("Invalid index URL")
});

pub(crate) struct JavaPlugin;

impl JavaPlugin {
    pub(crate) fn new(_moniker: &str) -> Plugin {
        Plugin::new(Self)
    }
}

impl PluginOps for JavaPlugin {
    fn url(&self) -> &Url {
        &INDEX_URL
    }

    fn parse_version(&self, s: &str) -> Result<Version> {
        Ok(Version::new(s.parse::<JavaVersion>()?))
    }

    fn make_env_info(&self, dir: &Path) -> EnvInfo {
        EnvInfo {
            path_dirs: vec![dir.join("bin")],
            vars: vec![],
        }
    }

    fn make_script_command(
        &self,
        _script_path: &Path,
        _platform: Platform,
        _shell: Shell,
    ) -> Result<Option<OsString>> {
        todo!()
    }

    fn new_package_manager(&self, ctx: PackageManagerContext) -> PackageManager {
        PackageManager::new(JavaPackageManager::new(ctx, &INDEX_URL))
    }
}
