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
use crate::go_package::GoPackage;
use crate::go_version::GoVersion;
use crate::serialization::Release;
use anyhow::{bail, Result};
use async_trait::async_trait;
use isopy_lib::{
    dir_url, install_package_error, DownloadFileOptions, DownloadPackageOptions,
    InstallPackageError, InstallPackageOptions, ListPackagesOptions, ListTagsOptions, Package,
    PackageKind, PackageManagerContext, PackageManagerOps, PackageSummary, SourceFilter, TagFilter,
    Tags, UpdateIndexOptions, Version,
};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;
use std::result::Result as StdResult;
use tokio::fs::read_to_string;
use url::Url;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const DEFAULT_TAGS: [&str; 2] = ["arm64", "darwin"];

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const DEFAULT_TAGS: [&str; 2] = ["amd64", "darwin"];

#[allow(unused)]
macro_rules! downcast_version {
    ($version : expr) => {
        $version
            .as_any()
            .downcast_ref::<$crate::go_version::GoVersion>()
            .ok_or_else(|| ::anyhow::anyhow!("Invalid version type"))?
    };
}

pub(crate) struct GoPackageManager {
    ctx: PackageManagerContext,
    url: Url,
}

impl GoPackageManager {
    pub(crate) fn new(ctx: PackageManagerContext, url: &Url) -> Self {
        Self {
            ctx,
            url: url.clone(),
        }
    }

    async fn get_index(&self, update: bool, show_progress: bool) -> Result<Value> {
        let options = DownloadFileOptions::json()
            .update(update)
            .show_progress(show_progress)
            .query(&[("include", "all"), ("mode", "json")]);
        let url = dir_url(&self.url);
        let path = self.ctx.download_file(&url, &options).await?;
        let s = read_to_string(path).await?;
        let index = serde_json::from_str(&s)?;
        Ok(index)
    }

    async fn get_packages(&self, update: bool, show_progress: bool) -> Result<Vec<GoPackage>> {
        let index = self.get_index(update, show_progress).await?;
        let mut packages = Vec::new();
        for release in serde_json::from_value::<Vec<Release>>(index)? {
            packages.extend(
                release
                    .files
                    .into_iter()
                    .map(GoPackage::from_file)
                    .collect::<Result<Vec<_>>>()?,
            );
        }
        Ok(packages)
    }

    async fn get_package(
        &self,
        update: bool,
        show_progress: bool,
        version: &GoVersion,
        _tags: &TagFilter,
    ) -> Result<GoPackage> {
        let packages = self.get_packages(update, show_progress).await?;
        let mut packages = packages
            .into_iter()
            .filter(|p| p.go_version() == version)
            .collect::<Vec<_>>();
        if packages.is_empty() {
            bail!("No matching packages found")
        }

        packages.sort_by_cached_key(|p| p.go_version().clone());
        packages.reverse();
        Ok(packages
            .into_iter()
            .next()
            .expect("Vector must contain at least one element"))
    }
}

#[async_trait]
impl PackageManagerOps for GoPackageManager {
    async fn update_index(&self, options: &UpdateIndexOptions) -> Result<()> {
        self.get_index(true, options.show_progress).await?;
        Ok(())
    }

    async fn list_tags(&self, _options: &ListTagsOptions) -> Result<Tags> {
        todo!()
    }

    async fn list_packages(
        &self,
        sources: SourceFilter,
        _tags: &TagFilter, // TBD
        options: &ListPackagesOptions,
    ) -> Result<Vec<PackageSummary>> {
        let filter_tags = HashSet::from(DEFAULT_TAGS);
        let packages = self.get_packages(false, options.show_progress).await?;
        let mut file_infos = Vec::new();
        for package in packages {
            let file = package.file;
            match file.kind.as_str() {
                "archive" => {
                    let tags = [file.arch.as_str(), file.os.as_str()]
                        .into_iter()
                        .collect::<HashSet<_>>();
                    if tags.is_superset(&filter_tags) {
                        let version = file.version.parse::<GoVersion>()?;
                        let url = self.url.join(&file.file_name)?;
                        let (kind, path) = match self.ctx.get_file(&url).await {
                            Ok(p) => (PackageKind::Local, Some(p)),
                            _ => (PackageKind::Remote, None),
                        };
                        let is_local = kind == PackageKind::Local;
                        match sources {
                            SourceFilter::All => {
                                file_infos.push((file.file_name, version, url, path));
                            }
                            SourceFilter::Local if is_local => {
                                file_infos.push((file.file_name, version, url, path));
                            }
                            SourceFilter::Remote if !is_local => {
                                file_infos.push((file.file_name, version, url, path));
                            }
                            _ => {}
                        }
                    }
                }
                "installer" | "source" => {}
                _ => todo!("Unimplemented file kind {}", file.kind),
            };
        }

        file_infos.sort_by(|a, b| a.1.cmp(&b.1));
        file_infos.reverse();

        let package_summaries = file_infos
            .into_iter()
            .map(|i| {
                PackageSummary::new(
                    PackageKind::Remote,
                    i.0,
                    &i.2,
                    Version::new(i.1),
                    None::<String>,
                    i.3,
                )
            })
            .collect::<Vec<_>>();
        Ok(package_summaries)
    }

    async fn download_package(
        &self,
        _version: &Version,
        _tags: &TagFilter,
        _options: &DownloadPackageOptions,
    ) -> Result<()> {
        todo!()
    }

    async fn install_package(
        &self,
        version: &Version,
        tags: &TagFilter,
        _dir: &Path,
        options: &InstallPackageOptions,
    ) -> StdResult<Package, InstallPackageError> {
        let version = downcast_version!(version);
        let Ok(package) = self
            .get_package(false, options.show_progress, version, tags)
            .await
        else {
            match tags {
                Some(tags) => {
                    log::error!("Release {} not found (tags {:?})", version, tags);
                }
                None => log::error!("Release {} not found", version),
            }
            return Err(InstallPackageError::VersionNotFound);
        };

        let package_path = self
            .ctx
            .get_file(
                &self
                    .url
                    .join(&package.file.file_name)
                    .map_err(|e| install_package_error!(e))?,
            )
            .await
            .map_err(|_| InstallPackageError::PackageNotDownloaded)?;

        todo!("package_path={package_path:?}");
        /*
        release
            .metadata()
            .archive_type()
            .unpack(&package_path, dir, options)
            .await?;
        */
        //Self::on_after_install(dir)?;
        //Ok(Package::new(package))
    }
}
