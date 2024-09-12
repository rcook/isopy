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
use crate::tng::python_package_manager::PythonPackageManager;
use anyhow::Result;
use isopy_lib::tng::{
    EnvProps, PackageManager, PackageManagerContext, Plugin, PluginOps, Version, VersionTriple,
};
use isopy_lib::EnvInfo;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use url::Url;

const INDEX_URL: LazyLock<Url> = LazyLock::new(|| {
    "https://api.github.com/repos/indygreg/python-build-standalone/releases"
        .parse()
        .expect("Invalid index URL")
});

pub(crate) struct PythonPlugin {
    url: Url,
}

impl PythonPlugin {
    pub(crate) fn new() -> Plugin {
        Plugin::new(Self {
            url: INDEX_URL.clone(),
        })
    }
}

impl PluginOps for PythonPlugin {
    fn url(&self) -> &Url {
        &self.url
    }

    fn parse_version(&self, s: &str) -> Result<Version> {
        Ok(Version::new(s.parse::<VersionTriple>()?))
    }

    fn make_env_info(
        &self,
        data_dir: &Path,
        env_props: &EnvProps,
        _base_dir: Option<&Path>,
    ) -> EnvInfo {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        fn make_path_dirs(data_dir: &Path, env_props: &EnvProps) -> Vec<PathBuf> {
            vec![data_dir.join(&env_props.dir()).join("bin")]
        }

        #[cfg(target_os = "windows")]
        fn make_path_dirs(data_dir: &Path, env_props: &EnvProps) -> Vec<PathBuf> {
            let d = data_dir.join(&env_props.dir());
            vec![d.clone(), d.join("Scripts")]
        }

        let path_dirs = make_path_dirs(data_dir, &env_props);
        /*
        let vars = if let Some(d) = base_dir {
            vec![(
                String::from("PYTHONPATH"),
                String::from(
                    d.to_str()
                        .ok_or_else(|| anyhow!("could not convert directory"))?,
                ),
            )]
        } else {
            vec![]
        };
        */
        let vars = vec![];

        EnvInfo { path_dirs, vars }
    }

    fn new_package_manager(&self, ctx: PackageManagerContext) -> PackageManager {
        PackageManager::new(PythonPackageManager::new(ctx, &self.url))
    }
}
