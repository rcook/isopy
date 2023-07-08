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
use crate::constants::{ADOPTIUM_SERVER_URL, PLUGIN_NAME};
use crate::openjdk_descriptor::OpenJdkDescriptor;
use crate::openjdk_plugin::OpenJdkPlugin;
use crate::serialization::{EnvConfigRec, ProjectConfigRec};
use anyhow::anyhow;
use isopy_lib::{
    other_error as isopy_lib_other_error, Descriptor, EnvInfo, IsopyLibResult, Plugin,
    PluginFactory,
};
use serde_json::Value;
use std::path::{Path, PathBuf};
use url::Url;

pub struct OpenJdkPluginFactory;

impl Default for OpenJdkPluginFactory {
    fn default() -> Self {
        Self
    }
}

impl PluginFactory for OpenJdkPluginFactory {
    fn name(&self) -> &str {
        PLUGIN_NAME
    }

    fn source_url(&self) -> &Url {
        &ADOPTIUM_SERVER_URL
    }

    fn read_project_config(&self, props: &Value) -> IsopyLibResult<Box<dyn Descriptor>> {
        let project_config_rec = serde_json::from_value::<ProjectConfigRec>(props.clone())
            .map_err(isopy_lib_other_error)?;
        Ok(Box::new(project_config_rec.descriptor))
    }

    fn parse_descriptor(&self, s: &str) -> IsopyLibResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<OpenJdkDescriptor>()
                .map_err(isopy_lib_other_error)?,
        ))
    }

    fn make_env_info(&self, data_dir: &Path, props: &Value) -> IsopyLibResult<EnvInfo> {
        #[cfg(target_os = "macos")]
        fn make_path_dirs(data_dir: &Path, env_config_rec: &EnvConfigRec) -> Vec<PathBuf> {
            vec![data_dir
                .join(&env_config_rec.dir)
                .join("Contents")
                .join("Home")
                .join("bin")]
        }

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        fn make_path_dirs(data_dir: &Path, env_config_rec: &EnvConfigRec) -> Vec<PathBuf> {
            vec![data_dir.join(&env_config_rec.dir).join("bin")]
        }

        let env_config_rec =
            serde_json::from_value::<EnvConfigRec>(props.clone()).map_err(isopy_lib_other_error)?;

        let openjdk_dir = data_dir.join(&env_config_rec.dir);
        let openjdk_dir_str = String::from(
            openjdk_dir
                .to_str()
                .ok_or_else(|| anyhow!("could not convert path to string"))?,
        );

        Ok(EnvInfo {
            path_dirs: make_path_dirs(data_dir, &env_config_rec),
            envs: vec![(String::from("JAVA_HOME"), openjdk_dir_str)],
        })
    }

    fn make_plugin(&self, dir: &Path) -> Box<dyn Plugin> {
        Box::new(OpenJdkPlugin::new(dir))
    }
}
