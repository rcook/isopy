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
use crate::adoptium::AdoptiumIndexManager;
use crate::constants::{ADOPTIUM_SERVER_URL, PLUGIN_NAME, PROJECT_CONFIG_FILE_NAME};
use crate::openjdk_descriptor::OpenJdkDescriptor;
use crate::serialization::{EnvConfigRec, ProjectConfigRec};
use anyhow::anyhow;
use async_trait::async_trait;
use isopy_lib::{
    other_error as isopy_lib_other_error, verify_sha256_file_checksum, Descriptor, EnvInfo,
    IsopyLibError, IsopyLibResult, Package, Product,
};
use joatmon::read_yaml_file;
use log::info;
use std::fs::{read_dir, remove_file};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use url::Url;

pub struct OpenJdk;

impl Default for OpenJdk {
    fn default() -> Self {
        Self
    }
}

impl OpenJdk {
    async fn download_openjdk(
        &self,
        descriptor: &OpenJdkDescriptor,
        shared_dir: &Path,
    ) -> IsopyLibResult<PathBuf> {
        let manager = AdoptiumIndexManager::new_default(shared_dir);

        let versions = manager.read_versions().await?;
        let Some(version) = versions
            .iter()
            .find(|x| x.openjdk_version == descriptor.version) else {
            return Err(IsopyLibError::VersionNotFound(descriptor.version.to_string()));
        };

        let asset_path = shared_dir.join(&version.file_name);
        if asset_path.exists() {
            info!("Asset {} already downloaded", version.file_name.display());
            return Ok(asset_path);
        }

        manager.download_asset(&version.url, &asset_path).await?;

        let is_valid = verify_sha256_file_checksum(&version.checksum, &asset_path)?;
        if !is_valid {
            remove_file(&asset_path).map_err(isopy_lib_other_error)?;
            return Err(IsopyLibError::ChecksumValidationFailed(asset_path));
        }

        info!(
            "SHA256 checksum validation succeeded on {}",
            asset_path.display()
        );

        Ok(asset_path)
    }
}

#[async_trait]
impl Product for OpenJdk {
    fn name(&self) -> &str {
        PLUGIN_NAME
    }

    fn repository_url(&self) -> &Url {
        &ADOPTIUM_SERVER_URL
    }

    async fn get_available_packages(&self, shared_dir: &Path) -> IsopyLibResult<Vec<Package>> {
        let manager = AdoptiumIndexManager::new_default(shared_dir);
        Ok(manager
            .read_versions()
            .await?
            .into_iter()
            .map(|x| Package {
                descriptor: Arc::new(Box::new(OpenJdkDescriptor {
                    version: x.openjdk_version.clone(),
                })),
                file_name: x.file_name,
            })
            .collect::<Vec<_>>())
    }

    fn project_config_file_name(&self) -> &Path {
        &PROJECT_CONFIG_FILE_NAME
    }

    fn read_project_config_file(&self, path: &Path) -> IsopyLibResult<Box<dyn Descriptor>> {
        Ok(Box::new(OpenJdkDescriptor {
            version: read_yaml_file::<ProjectConfigRec>(path)
                .map_err(isopy_lib_other_error)?
                .version,
        }))
    }

    fn parse_descriptor(&self, s: &str) -> IsopyLibResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<OpenJdkDescriptor>()
                .map_err(isopy_lib_other_error)?,
        ))
    }

    async fn download_asset(
        &self,
        descriptor: &dyn Descriptor,
        shared_dir: &Path,
    ) -> IsopyLibResult<PathBuf> {
        let descriptor = descriptor
            .as_any()
            .downcast_ref::<OpenJdkDescriptor>()
            .expect("must be OpenJdkDescriptor");
        self.download_openjdk(descriptor, shared_dir).await
    }

    fn read_env_config(
        &self,
        data_dir: &Path,
        properties: &serde_json::Value,
    ) -> IsopyLibResult<EnvInfo> {
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

        let env_config_rec = serde_json::from_value::<EnvConfigRec>(properties.clone())
            .map_err(isopy_lib_other_error)?;

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

    fn get_downloaded(&self, shared_dir: &Path) -> IsopyLibResult<Vec<PathBuf>> {
        let mut asset_file_names = Vec::new();
        for result in read_dir(shared_dir).map_err(isopy_lib_other_error)? {
            let entry = result.map_err(isopy_lib_other_error)?;
            let asset_file_name = entry.file_name();
            if let Some(asset_file_name) = asset_file_name.to_str() {
                if asset_file_name.starts_with("OpenJDK") {
                    asset_file_names.push(PathBuf::from(asset_file_name));
                }
            }
        }

        Ok(asset_file_names)
    }
}
