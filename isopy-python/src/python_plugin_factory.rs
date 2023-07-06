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
use crate::constants::{PLUGIN_NAME, PROJECT_CONFIG_FILE_NAME, RELEASES_URL};
use crate::python_descriptor::PythonDescriptor;
use crate::python_plugin::PythonPlugin;
use crate::serialization::{EnvConfigRec, ProjectConfigRec};
use isopy_lib::{
    other_error as isopy_lib_other_error, Descriptor, EnvInfo, IsopyLibResult, Plugin,
    PluginFactory,
};
use joatmon::read_yaml_file;
use std::ffi::OsStr;
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

    fn project_config_file_name(&self) -> &OsStr {
        &PROJECT_CONFIG_FILE_NAME
    }

    fn read_project_config_file(&self, path: &Path) -> IsopyLibResult<Box<dyn Descriptor>> {
        let project_config_rec =
            read_yaml_file::<ProjectConfigRec>(path).map_err(isopy_lib_other_error)?;

        Ok(Box::new(PythonDescriptor {
            version: project_config_rec.version,
            tag: project_config_rec.tag,
        }))
    }

    fn parse_descriptor(&self, s: &str) -> IsopyLibResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<PythonDescriptor>()
                .map_err(isopy_lib_other_error)?,
        ))
    }

    fn read_env_config(
        &self,
        data_dir: &Path,
        properties: &serde_json::Value,
    ) -> IsopyLibResult<EnvInfo> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        fn make_path_dirs(data_dir: &Path, env_config_rec: &EnvConfigRec) -> Vec<PathBuf> {
            vec![data_dir.join(&env_config_rec.dir).join("bin")]
        }

        #[cfg(target_os = "windows")]
        fn make_path_dirs(data_dir: &Path, env_config_rec: &EnvConfigRec) -> Vec<PathBuf> {
            vec![
                data_dir.join(&env_config_rec.dir).join("bin"),
                data_dir.join(&env_config_rec.dir).join("Scripts"),
            ]
        }

        let env_config_rec = serde_json::from_value::<EnvConfigRec>(properties.clone())
            .map_err(isopy_lib_other_error)?;

        Ok(EnvInfo {
            path_dirs: make_path_dirs(data_dir, &env_config_rec),
            envs: vec![],
        })
    }

    fn make_plugin(&self, dir: &Path) -> Box<dyn Plugin> {
        Box::new(PythonPlugin::new(dir))
    }
}
