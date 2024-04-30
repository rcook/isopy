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
use crate::asset::Asset;
use crate::asset_filter::AssetFilter;
use crate::asset_helper::{download_asset, get_asset};
use crate::asset_meta::AssetMeta;
use crate::constants::{
    ASSETS_DIR, INDEX_FILE_NAME, RELEASES_FILE_NAME, RELEASES_URL, REPOSITORIES_FILE_NAME,
};
use crate::github::GitHubRepository;
use crate::local::LocalRepository;
use crate::python_descriptor::PythonDescriptor;
use crate::python_version::PythonVersion;
use crate::repository::Repository;
use crate::repository_info::RepositoryInfo;
use crate::repository_name::RepositoryName;
use crate::serialization::{IndexRec, PackageRec, RepositoriesRec, RepositoryRec};
use crate::tag::Tag;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use isopy_lib::{
    dir_url, download_stream, other_error as isopy_lib_other_error, Descriptor, IsopyLibResult,
    LastModified, Package, Plugin,
};
use joatmon::label_file_name;
use joatmon::read_yaml_file;
use joatmon::{read_json_file, safe_write_file};
use log::trace;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct PythonPlugin {
    offline: bool,
    dir: PathBuf,
    assets_dir: PathBuf,
}

impl PythonPlugin {
    pub fn new(offline: bool, dir: &Path) -> Self {
        let dir = dir.to_path_buf();
        let assets_dir = dir.join(&*ASSETS_DIR);
        Self {
            offline,
            dir,
            assets_dir,
        }
    }

    fn read_assets(&self) -> Result<Vec<Asset>> {
        let index_path = self.dir.join(RELEASES_FILE_NAME);
        let package_recs = read_json_file::<Vec<PackageRec>>(&index_path)?;

        let mut assets = Vec::new();
        for package_rec in package_recs {
            for asset_rec in package_rec.assets {
                if !AssetMeta::definitely_not_an_asset_name(&asset_rec.name) {
                    let meta = asset_rec.name.parse::<AssetMeta>()?;
                    assets.push(Asset {
                        name: asset_rec.name,
                        tag: package_rec.tag.clone(),
                        url: asset_rec.url,
                        size: asset_rec.size,
                        meta,
                    });
                }
            }
        }
        Ok(assets)
    }

    fn read_repositories(&self) -> Result<Vec<RepositoryInfo>> {
        fn make_repository(
            offline: bool,
            rec: RepositoryRec,
        ) -> (RepositoryName, bool, Box<dyn Repository>) {
            match rec {
                RepositoryRec::GitHub { name, url, enabled } => (
                    name,
                    enabled,
                    Box::new(GitHubRepository::new(offline, &url)),
                ),
                RepositoryRec::Local { name, dir, enabled } => {
                    (name, enabled, Box::new(LocalRepository::new(&dir)))
                }
            }
        }

        let repositories_yaml_path = self.dir.join(REPOSITORIES_FILE_NAME);
        let repositories_rec = if repositories_yaml_path.is_file() {
            read_yaml_file::<RepositoriesRec>(&repositories_yaml_path)?
        } else {
            let repositories_rec = RepositoriesRec {
                repositories: vec![
                    RepositoryRec::GitHub {
                        name: RepositoryName::Default,
                        url: dir_url(&RELEASES_URL),
                        enabled: true,
                    },
                    RepositoryRec::Local {
                        name: RepositoryName::Example,
                        dir: PathBuf::from("/path/to/local/repository"),
                        enabled: false,
                    },
                ],
            };
            safe_write_file(
                &repositories_yaml_path,
                serde_yaml::to_string(&repositories_rec)?,
                false,
            )?;
            repositories_rec
        };

        let all_repositories = repositories_rec
            .repositories
            .into_iter()
            .map(|rec| make_repository(self.offline, rec));
        let enabled_repositories = all_repositories
            .into_iter()
            .filter(|x| x.1)
            .map(|x| RepositoryInfo {
                name: x.0,
                repository: x.2,
            })
            .collect::<Vec<_>>();
        Ok(enabled_repositories)
    }

    async fn update_index_if_necessary(&self) -> Result<()> {
        let repositories = self.read_repositories()?;
        let repository = repositories
            .first()
            .ok_or_else(|| anyhow!("No asset repositories are configured"))?;

        let releases_path = self.releases_path(&repository.name);
        let current_last_modified = if releases_path.is_file() {
            self.read_index_last_modified(&repository.name)?
        } else {
            None
        };

        if let Some(mut response) = repository
            .repository
            .get_latest_index(&current_last_modified)
            .await?
        {
            download_stream("release index", &mut response, &releases_path).await?;
            if let Some(last_modified) = response.last_modified() {
                self.write_index_last_modified(&repository.name, last_modified)?;
            }
        }

        Ok(())
    }

    fn releases_path(&self, repository_name: &RepositoryName) -> PathBuf {
        if repository_name.is_default() {
            self.dir.join(RELEASES_FILE_NAME)
        } else {
            label_file_name(&self.dir.join(RELEASES_FILE_NAME), repository_name.as_str())
                .expect("must be valid")
        }
    }

    fn read_index_last_modified(
        &self,
        repository_name: &RepositoryName,
    ) -> Result<Option<LastModified>> {
        let index_path = self.index_path(repository_name);
        Ok(if index_path.is_file() {
            Some(read_yaml_file::<IndexRec>(&index_path)?.last_modified)
        } else {
            None
        })
    }

    fn write_index_last_modified(
        &self,
        repository_name: &RepositoryName,
        last_modified: &LastModified,
    ) -> Result<()> {
        let index_yaml_path = self.index_path(repository_name);
        safe_write_file(
            &index_yaml_path,
            serde_yaml::to_string(&IndexRec {
                last_modified: last_modified.clone(),
            })?,
            true,
        )?;
        Ok(())
    }

    fn index_path(&self, repository_name: &RepositoryName) -> PathBuf {
        if repository_name.is_default() {
            self.dir.join(INDEX_FILE_NAME)
        } else {
            label_file_name(&self.dir.join(INDEX_FILE_NAME), repository_name.as_str())
                .expect("must be valid")
        }
    }

    async fn get_available_packages_extended(
        &self,
    ) -> IsopyLibResult<Vec<(Package, PythonVersion, Tag)>> {
        self.update_index_if_necessary().await?;

        let mut assets = self.read_assets()?;
        assets.sort_by(|a, b| Self::compare_assets_by_version_and_tag(b, a));

        let packages_with_version = AssetFilter::default_for_platform()
            .filter(assets.iter())
            .into_iter()
            .map(|asset| {
                (
                    Package {
                        asset_path: self.assets_dir.join(asset.name.clone()),
                        descriptor: Arc::new(Box::new(PythonDescriptor {
                            version: asset.meta.version.clone(),
                            tag: Some(asset.tag.clone()),
                        })),
                    },
                    asset.meta.version.clone(),
                    asset.tag.clone(),
                )
            })
            .collect::<Vec<_>>();

        Ok(packages_with_version)
    }

    fn compare_assets_by_version_and_tag(a: &Asset, b: &Asset) -> Ordering {
        Self::compare_by_version_and_tag((&a.meta.version, &a.tag), (&b.meta.version, &b.tag))
    }

    fn compare_by_version_and_tag(
        a: (&PythonVersion, &Tag),
        b: (&PythonVersion, &Tag),
    ) -> Ordering {
        match a.0.cmp(b.0) {
            Ordering::Equal => a.1.cmp(b.1),
            result => result,
        }
    }
}

#[async_trait]
impl Plugin for PythonPlugin {
    async fn get_available_packages(&self) -> IsopyLibResult<Vec<Package>> {
        let items = self.get_available_packages_extended().await?;
        Ok(items.into_iter().map(|p| p.0).collect::<Vec<_>>())
    }

    async fn get_downloaded_packages(&self) -> IsopyLibResult<Vec<Package>> {
        let packages_with_version = self.get_available_packages_extended().await?;
        let map = packages_with_version
            .iter()
            .filter_map(|p| {
                p.0.asset_path
                    .file_name()
                    .map(OsString::from)
                    .map(|f| (f, p))
            })
            .collect::<HashMap<_, _>>();

        let mut items = Vec::new();

        if self.assets_dir.exists() {
            for result in read_dir(&self.assets_dir).map_err(isopy_lib_other_error)? {
                let entry = result.map_err(isopy_lib_other_error)?;
                let asset_path = entry.path();
                let asset_file_name = entry.file_name();
                if let Some(p) = map
                    .get(&asset_file_name)
                    .map(|p| (Arc::clone(&p.0.descriptor), &p.1, &p.2))
                {
                    if let Some(s) = asset_file_name.to_str() {
                        if s.parse::<AssetMeta>().is_ok() {
                            items.push((
                                Package {
                                    asset_path,
                                    descriptor: p.0,
                                },
                                p.1,
                                p.2,
                            ));
                        }
                    }
                }
            }
        }

        items.sort_by(|a, b| Self::compare_by_version_and_tag((b.1, b.2), (a.1, a.2)));

        Ok(items.into_iter().map(|p| p.0).collect::<Vec<_>>())
    }

    async fn download_package(&self, descriptor: &dyn Descriptor) -> IsopyLibResult<Package> {
        let descriptor = descriptor
            .as_any()
            .downcast_ref::<PythonDescriptor>()
            .expect("must be PythonDescriptor");
        let assets = self.read_assets()?;
        let asset = get_asset(&assets, descriptor)?;
        let repositories = self.read_repositories()?;
        let repository = repositories
            .first()
            .ok_or_else(|| anyhow!("No asset repositories are configured"))?;
        let asset_path = download_asset(repository, asset, &self.assets_dir).await?;

        Ok(Package {
            asset_path,
            descriptor: Arc::new(Box::new(PythonDescriptor {
                version: descriptor.version.clone(),
                tag: Some(asset.tag.clone()),
            })),
        })
    }

    async fn on_before_install(
        &self,
        _output_dir: &Path,
        _bin_subdir: &Path,
    ) -> IsopyLibResult<()> {
        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    async fn on_after_install(&self, output_dir: &Path, bin_subdir: &Path) -> IsopyLibResult<()> {
        use std::os::unix::fs::symlink;

        let bin_dir = output_dir.join(bin_subdir).join("bin");
        let link = bin_dir.join("python");
        if !link.exists() {
            let original = bin_dir.join("python3");
            trace!("Creating link {} to {}", link.display(), original.display());
            symlink(&original, &link).map_err(isopy_lib_other_error)?;
            trace!("Created link {} to {}", link.display(), original.display());
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    async fn on_after_install(&self, output_dir: &Path, bin_subdir: &Path) -> IsopyLibResult<()> {
        use std::fs::write;

        let cmd_path = output_dir.join(bin_subdir).join("python3.cmd");
        if !cmd_path.exists() {
            const WRAPPER: &str = "@echo off\n\"%~dp0python.exe\" %*\n";
            trace!("Creating wrapper script {}", cmd_path.display());
            write(&cmd_path, WRAPPER).map_err(isopy_lib_other_error)?;
            trace!("Created wrapper script {}", cmd_path.display());
        }
        Ok(())
    }
}
