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
use crate::adoptium::api::ImageType;
use crate::adoptium::AdoptiumIndexManager;
use crate::constants::ASSETS_DIR;
use crate::java_descriptor::JavaDescriptor;
use crate::java_version::JavaVersion;
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

pub struct JavaPlugin {
    offline: bool,
    image_type: ImageType,
    dir: PathBuf,
    assets_dir: PathBuf,
}

struct PackageInfo {
    package: Package,
    version: JavaVersion,
}

impl JavaPlugin {
    pub fn new(offline: bool, image_type: ImageType, dir: &Path) -> Self {
        let dir = dir.to_path_buf();
        let assets_dir = dir.join(&*ASSETS_DIR);
        Self {
            offline,
            image_type,
            dir,
            assets_dir,
        }
    }

    async fn get_available_package_infos(&self) -> IsopyLibResult<Vec<PackageInfo>> {
        Ok(
            AdoptiumIndexManager::new_default(self.offline, self.image_type.clone(), &self.dir)
                .read_versions()
                .await?
                .into_iter()
                .map(|x| PackageInfo {
                    package: Package {
                        asset_path: self.assets_dir.join(x.file_name),
                        descriptor: Arc::new(Box::new(JavaDescriptor {
                            version: x.openjdk_version.clone(),
                        })),
                    },
                    version: x.openjdk_version,
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[async_trait]
impl Plugin for JavaPlugin {
    async fn get_available_packages(&self) -> IsopyLibResult<Vec<Package>> {
        let infos = self.get_available_package_infos().await?;
        Ok(infos.into_iter().map(|x| x.package).collect::<Vec<_>>())
    }

    async fn get_downloaded_packages(&self) -> IsopyLibResult<Vec<Package>> {
        if self.assets_dir.exists() {
            let infos = self.get_available_package_infos().await?;
            let map = infos
                .iter()
                .filter_map(|x| {
                    x.package
                        .asset_path
                        .file_name()
                        .map(OsString::from)
                        .map(|f| (f, x))
                })
                .collect::<HashMap<_, _>>();

            let mut items = Vec::new();
            for result in read_dir(&self.assets_dir).map_err(isopy_lib_other_error)? {
                let entry = result.map_err(isopy_lib_other_error)?;
                let asset_path = entry.path();
                let asset_file_name = entry.file_name();
                if let Some(x) = map
                    .get(&asset_file_name)
                    .map(|x| (Arc::clone(&x.package.descriptor), &x.version))
                {
                    items.push((
                        Package {
                            asset_path,
                            descriptor: x.0,
                        },
                        x.1,
                    ));
                }
            }

            items.sort_by(|a, b| b.1.cmp(a.1));
            Ok(items.into_iter().map(|p| p.0).collect::<Vec<_>>())
        } else {
            Ok(vec![])
        }
    }

    async fn download_package(&self, descriptor: &dyn Descriptor) -> IsopyLibResult<Package> {
        let descriptor = descriptor
            .as_any()
            .downcast_ref::<JavaDescriptor>()
            .expect("must be JavaDescriptor");

        let manager =
            AdoptiumIndexManager::new_default(self.offline, self.image_type.clone(), &self.dir);

        let versions = manager.read_versions().await?;
        let Some(version) = versions
            .iter()
            .find(|x| x.openjdk_version == descriptor.version)
        else {
            return Err(IsopyLibError::VersionNotFound(
                descriptor.version.to_string(),
            ));
        };

        let asset_path = self.assets_dir.join(&version.file_name);
        if asset_path.exists() {
            info!("asset {} already downloaded", version.file_name);
            return Ok(Package {
                asset_path,
                descriptor: Arc::new(Box::new(descriptor.clone())),
            });
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

        Ok(Package {
            asset_path,
            descriptor: Arc::new(Box::new(descriptor.clone())),
        })
    }

    async fn on_before_install(
        &self,
        _output_dir: &Path,
        _bin_subdir: &Path,
    ) -> IsopyLibResult<()> {
        Ok(())
    }

    async fn on_after_install(&self, _output_dir: &Path, _bin_subdir: &Path) -> IsopyLibResult<()> {
        Ok(())
    }
}
