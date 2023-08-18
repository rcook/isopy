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
use crate::serialization::{IndexRec, VersionRec};
use async_trait::async_trait;
use isopy_lib::{
    download_stream, other_error as isopy_lib_other_error, verify_sha256_file_checksum, Descriptor,
    IsopyLibError, IsopyLibResult, Package, Plugin, ReqwestResponse, Response,
};
use joatmon::read_yaml_file;
use log::info;
use reqwest::Client;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct GoPlugin {
    _offline: bool,
    dir: PathBuf,
    assets_dir: PathBuf,
}

struct PackageInfo {
    package: Package,
    version_rec: VersionRec,
    version: GoVersion,
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

        let index_rec = read_yaml_file::<IndexRec>(&index_path).map_err(isopy_lib_other_error)?;

        let mut infos = Vec::new();
        for version_rec in index_rec.versions {
            let asset_path = self.assets_dir.join(&version_rec.file_name);
            let version = version_rec
                .version
                .parse::<GoVersion>()
                .map_err(isopy_lib_other_error)?;

            let descriptor: Arc<Box<dyn Descriptor>> = Arc::new(Box::new(version.clone()));

            let package = Package {
                asset_path,
                descriptor,
            };

            infos.push(PackageInfo {
                package,
                version_rec,
                version,
            });
        }

        infos.sort_by(|a, b| b.version.cmp(&a.version));

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
        let mut items: Vec<(Package, GoVersion)> = Vec::new();

        if self.assets_dir.exists() {
            let infos = self.get_available_package_infos().await?;
            for info in infos {
                println!("{}", info.version.raw);
            }
            todo!();
        }

        items.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(items.into_iter().map(|p| p.0).collect::<Vec<_>>())
    }

    async fn download_package(&self, descriptor: &dyn Descriptor) -> IsopyLibResult<Package> {
        let version = descriptor
            .as_any()
            .downcast_ref::<GoVersion>()
            .expect("must be GoVersion");

        let infos = self.get_available_package_infos().await?;
        let Some(item) = infos.iter().find(|x| x.version == *version) else {
            return Err(IsopyLibError::VersionNotFound(version.to_string()));
        };

        let asset_path = self.assets_dir.join(&item.version_rec.file_name);
        if asset_path.exists() {
            info!("asset {} already downloaded", item.version_rec.file_name);
            return Ok(Package {
                asset_path,
                descriptor: Arc::new(Box::new(version.clone())),
            });
        }

        let client = Client::new();
        let request = client.get(item.version_rec.url.clone());
        let response = request.send().await.map_err(isopy_lib_other_error)?;
        let mut wrapped: Box<dyn Response> = Box::new(ReqwestResponse::new(None, response));
        download_stream("Go asset", &mut wrapped, &asset_path).await?;

        let is_valid = verify_sha256_file_checksum(&item.version_rec.checksum, &asset_path)?;
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
}
