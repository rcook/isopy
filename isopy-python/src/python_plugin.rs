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
use crate::constants::{PYTHON_BIN_FILE_NAME, PYTHON_SCRIPT_EXT};
use crate::python_package_manager::PythonPackageManager;
use crate::python_version::PythonVersion;
use anyhow::Result;
use isopy_lib::{render_absolute_path, EnvInfo, Platform, Shell};
use isopy_lib::{PackageManager, PackageManagerContext, Plugin, PluginOps, Version};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use url::Url;

fn file_url(s: &str) -> Url {
    s.trim_end_matches('/').parse().expect("Invalid URL")
}

fn dir_url(s: &str) -> Url {
    let s = s.trim_end_matches('/');
    let mut s = String::from(s);
    s.push('/');
    s.parse().expect("Invalid URL")
}

static INDEX_URL: LazyLock<Url> = LazyLock::new(|| {
    file_url("https://api.github.com/repos/astral-sh/python-build-standalone/releases")
});

pub(crate) static CHECKSUM_BASE_URL: LazyLock<Url> =
    LazyLock::new(|| dir_url("https://blog.rcook.org/assets/isopy"));

pub(crate) struct PythonPlugin {
    moniker: String,
}

impl PythonPlugin {
    pub(crate) fn new(moniker: &str) -> Plugin {
        Plugin::new(Self {
            moniker: String::from(moniker),
        })
    }
}

impl PluginOps for PythonPlugin {
    fn url(&self) -> &Url {
        &INDEX_URL
    }

    fn parse_version(&self, s: &str) -> Result<Version> {
        Ok(Version::new(s.parse::<PythonVersion>()?))
    }

    fn make_env_info(&self, dir: &Path) -> EnvInfo {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        fn make_path_dirs(dir: &Path) -> Vec<PathBuf> {
            vec![dir.join("bin")]
        }

        #[cfg(target_os = "windows")]
        fn make_path_dirs(dir: &Path) -> Vec<PathBuf> {
            vec![dir.to_path_buf(), dir.join("Scripts")]
        }

        let path_dirs = make_path_dirs(dir);
        let vars = vec![];
        EnvInfo { path_dirs, vars }
    }

    fn make_script_command(
        &self,
        script_path: &Path,
        _platform: Platform,
        shell: Shell,
    ) -> Result<Option<OsString>> {
        fn make_command(script_path: &Path, shell: Shell) -> Result<OsString> {
            let delimiter: &str = match shell {
                Shell::Bash => "'",
                Shell::Cmd => "\"",
            };

            let mut s = OsString::new();
            s.push(PYTHON_BIN_FILE_NAME.as_os_str());
            s.push(" ");
            s.push(delimiter);
            s.push(render_absolute_path(shell, script_path)?);
            s.push(delimiter);
            Ok(s)
        }

        if script_path
            .extension()
            .map(OsStr::to_ascii_lowercase)
            .as_ref()
            == Some(&*PYTHON_SCRIPT_EXT)
        {
            Ok(Some(make_command(script_path, shell)?))
        } else {
            Ok(None)
        }
    }

    fn new_package_manager(&self, ctx: PackageManagerContext) -> PackageManager {
        PackageManager::new(PythonPackageManager::new(ctx, &self.moniker, &INDEX_URL))
    }
}
