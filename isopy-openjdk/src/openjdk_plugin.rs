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
use crate::constants::ASSETS_DIR;
use crate::openjdk_descriptor::OpenJdkDescriptor;
use async_trait::async_trait;
use isopy_lib::{
    other_error as isopy_lib_other_error, verify_sha256_file_checksum, Descriptor, IsopyLibError,
    IsopyLibResult, Package, Plugin,
};
use log::info;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{read_dir, remove_file};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct OpenJdkPlugin {
    dir: PathBuf,
}

impl OpenJdkPlugin {
    pub fn new(dir: &Path) -> Self {
        Self {
            dir: dir.to_path_buf(),
        }
    }
}

#[async_trait]
impl Plugin for OpenJdkPlugin {
    async fn get_available_packages(&self) -> IsopyLibResult<Vec<Package>> {
        let manager = AdoptiumIndexManager::new_default(&self.dir);
        Ok(manager
            .read_versions()
            .await?
            .into_iter()
            .map(|x| Package {
                asset_path: self.dir.join(&*ASSETS_DIR).join(x.file_name),
                descriptor: Some(Arc::new(Box::new(OpenJdkDescriptor {
                    version: x.openjdk_version,
                }))),
            })
            .collect::<Vec<_>>())
    }

    async fn get_downloaded_packages(&self) -> IsopyLibResult<Vec<Package>> {
        let packages = self.get_available_packages().await?;
        let package_map = packages
            .iter()
            .filter_map(|p| p.asset_path.file_name().map(OsString::from).map(|f| (f, p)))
            .collect::<HashMap<_, _>>();

        let assets_dir = self.dir.join(&*ASSETS_DIR);
        let mut packages = Vec::new();
        for result in read_dir(assets_dir).map_err(isopy_lib_other_error)? {
            let entry = result.map_err(isopy_lib_other_error)?;
            let asset_path = entry.path();
            let asset_file_name = entry.file_name();
            let descriptor = package_map
                .get(&asset_file_name)
                .and_then(|package| package.descriptor.as_ref())
                .cloned();
            packages.push(Package {
                asset_path,
                descriptor,
            });
        }

        Ok(packages)
    }

    async fn download_asset(&self, descriptor: &dyn Descriptor) -> IsopyLibResult<PathBuf> {
        let descriptor = descriptor
            .as_any()
            .downcast_ref::<OpenJdkDescriptor>()
            .expect("must be OpenJdkDescriptor");

        let manager = AdoptiumIndexManager::new_default(&self.dir);

        let versions = manager.read_versions().await?;
        let Some(version) = versions
                    .iter()
                    .find(|x| x.openjdk_version == descriptor.version) else {
                    return Err(IsopyLibError::VersionNotFound(descriptor.version.to_string()));
                };

        let assets_dir = self.dir.join(&*ASSETS_DIR);
        let asset_path = assets_dir.join(&version.file_name);
        if asset_path.exists() {
            info!("asset {} already downloaded", version.file_name);
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
