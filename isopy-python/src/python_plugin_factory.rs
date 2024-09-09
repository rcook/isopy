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
use crate::constants::{PLUGIN_NAME, PYTHON_BIN_FILE_NAME, PYTHON_SCRIPT_EXT, RELEASES_URL};
use crate::python_descriptor::PythonDescriptor;
use crate::python_plugin::PythonPlugin;
use crate::serialization::{EnvConfig, ProjectConfig};
use isopy_lib::{
    other_error as isopy_lib_other_error, render_absolute_path, Descriptor, EnvInfo,
    IsopyLibResult, Platform, Plugin, PluginFactory, Shell,
};
use serde_json::Value;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use url::Url;

pub struct PythonPluginFactory;

impl Default for PythonPluginFactory {
    fn default() -> Self {
        Self
    }
}

impl PluginFactory for PythonPluginFactory {
    fn name(&self) -> &str {
        PLUGIN_NAME
    }

    fn source_url(&self) -> &Url {
        &RELEASES_URL
    }

    fn read_project_config(&self, props: &Value) -> IsopyLibResult<Box<dyn Descriptor>> {
        let project_config_rec = serde_json::from_value::<ProjectConfig>(props.clone())
            .map_err(isopy_lib_other_error)?;
        Ok(Box::new(project_config_rec.descriptor))
    }

    fn parse_descriptor(&self, s: &str) -> IsopyLibResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<PythonDescriptor>()
                .map_err(isopy_lib_other_error)?,
        ))
    }

    fn make_env_info(
        &self,
        data_dir: &Path,
        props: &Value,
        _base_dir: Option<&Path>,
    ) -> IsopyLibResult<EnvInfo> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        fn make_path_dirs(data_dir: &Path, env_config_rec: &EnvConfig) -> Vec<PathBuf> {
            vec![data_dir.join(&env_config_rec.dir).join("bin")]
        }

        #[cfg(target_os = "windows")]
        fn make_path_dirs(data_dir: &Path, env_config_rec: &EnvConfig) -> Vec<PathBuf> {
            vec![
                data_dir.join(&env_config_rec.dir),
                data_dir.join(&env_config_rec.dir).join("Scripts"),
            ]
        }

        let env_config_rec =
            serde_json::from_value::<EnvConfig>(props.clone()).map_err(isopy_lib_other_error)?;

        let path_dirs = make_path_dirs(data_dir, &env_config_rec);
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

        Ok(EnvInfo { path_dirs, vars })
    }

    fn make_script_command(
        &self,
        script_path: &Path,
        _platform: Platform,
        shell: Shell,
    ) -> IsopyLibResult<Option<OsString>> {
        fn make_command(script_path: &Path, shell: Shell) -> IsopyLibResult<OsString> {
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

    fn make_plugin(&self, offline: bool, dir: &Path) -> Box<dyn Plugin> {
        Box::new(PythonPlugin::new(offline, dir))
    }
}
