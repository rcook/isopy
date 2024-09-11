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
use crate::constants::{ASSETS_DIR, INDEX_FILE_NAME};
use crate::download::download_index;
use crate::filter::Filter;
use crate::go_version::GoVersion;
use crate::serialization::{Index, Version};
use async_trait::async_trait;
use isopy_lib::{
    other_error as isopy_lib_other_error, Descriptor, IsopyLibResult, Package, Plugin,
};
use joatmon::read_yaml_file;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct GoPlugin {
    _offline: bool,
    dir: PathBuf,
    assets_dir: PathBuf,
}

struct PackageInfo {
    package: Package,
    #[allow(unused)]
    version: Version,
    go_version: GoVersion,
}

impl GoPlugin {
    pub fn new(offline: bool, dir: &Path) -> Self {
        let assets_dir = dir.join(&*ASSETS_DIR);
        Self {
            _offline: offline,
            dir: dir.to_path_buf(),
            assets_dir,
        }
    }

    async fn get_available_package_infos(&self) -> IsopyLibResult<Vec<PackageInfo>> {
        let index_path = self.dir.join(&*INDEX_FILE_NAME);

        download_index(&Filter::default(), &index_path)
            .await
            .map_err(isopy_lib_other_error)?;

        let index = read_yaml_file::<Index>(&index_path).map_err(isopy_lib_other_error)?;

        let mut infos = Vec::new();
        for version in index.versions {
            let asset_path = self.assets_dir.join(&version.file_name);
            let go_version = version
                .version
                .parse::<GoVersion>()
                .map_err(isopy_lib_other_error)?;

            let descriptor: Arc<Box<dyn Descriptor>> = Arc::new(Box::new(go_version.clone()));

            let package = Package {
                asset_path,
                descriptor,
            };

            infos.push(PackageInfo {
                package,
                version,
                go_version,
            });
        }

        infos.sort_by(|a, b| b.go_version.cmp(&a.go_version));

        Ok(infos)
    }
}

#[async_trait]
impl Plugin for GoPlugin {
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
                    .map(|x| (Arc::clone(&x.package.descriptor), &x.go_version))
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

    /*
    async fn download_package(&self, descriptor: &dyn Descriptor) -> IsopyLibResult<Package> {
        let version = descriptor
            .as_any()
            .downcast_ref::<GoVersion>()
            .expect("must be GoVersion");

        let infos = self.get_available_package_infos().await?;
        let Some(item) = infos.iter().find(|x| x.go_version == *version) else {
            return Err(IsopyLibError::VersionNotFound(version.to_string()));
        };

        let asset_path = self.assets_dir.join(&item.version.file_name);
        if asset_path.exists() {
            info!("asset {} already downloaded", item.version.file_name);
            return Ok(Package {
                asset_path,
                descriptor: Arc::new(Box::new(version.clone())),
            });
        }

        let client = Client::new();
        let request = client.get(item.version.url.clone());
        let response = request.send().await.map_err(isopy_lib_other_error)?;
        let mut wrapped: Box<dyn Response> = Box::new(ReqwestResponse::new(None, response));
        download_stream("Go asset", &mut wrapped, &asset_path).await?;

        let is_valid = verify_sha256_file_checksum(&item.version.checksum, &asset_path)?;
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
            descriptor: Arc::new(Box::new(version.clone())),
        })
    }
    */
}
