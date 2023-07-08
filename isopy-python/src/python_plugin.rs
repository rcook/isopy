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
use crate::repository::Repository;
use crate::repository_info::RepositoryInfo;
use crate::repository_name::RepositoryName;
use crate::serialization::{IndexRec, PackageRec, RepositoriesRec, RepositoryRec};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use isopy_lib::{
    dir_url, download_stream, other_error as isopy_lib_other_error, Descriptor, IsopyLibResult,
    LastModified, Package, Plugin,
};
use joatmon::label_file_name;
use joatmon::read_yaml_file;
use joatmon::{read_json_file, safe_write_file};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct PythonPlugin {
    dir: PathBuf,
    assets_dir: PathBuf,
}

impl PythonPlugin {
    pub fn new(dir: &Path) -> Self {
        let dir = dir.to_path_buf();
        let assets_dir = dir.join(&*ASSETS_DIR);
        Self { dir, assets_dir }
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
        fn make_repository(rec: RepositoryRec) -> (RepositoryName, bool, Box<dyn Repository>) {
            match rec {
                RepositoryRec::GitHub { name, url, enabled } => {
                    (name, enabled, Box::new(GitHubRepository::new(&url)))
                }
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
            .map(make_repository);
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
}

#[async_trait]
impl Plugin for PythonPlugin {
    async fn get_available_packages(&self) -> IsopyLibResult<Vec<Package>> {
        fn compare_by_version_and_tag(a: &Asset, b: &Asset) -> Ordering {
            match a.meta.version.cmp(&b.meta.version) {
                Ordering::Equal => a.tag.cmp(&b.tag),
                result => result,
            }
        }

        self.update_index_if_necessary().await?;

        let mut assets = self.read_assets()?;
        assets.sort_by(|a, b| compare_by_version_and_tag(b, a));

        Ok(AssetFilter::default_for_platform()
            .filter(assets.iter())
            .into_iter()
            .map(|asset| Package {
                descriptor: Some(Arc::new(Box::new(PythonDescriptor {
                    version: asset.meta.version.clone(),
                    tag: Some(asset.tag.clone()),
                }))),
                asset_path: self.assets_dir.join(asset.name.clone()),
            })
            .collect::<Vec<_>>())
    }

    async fn get_downloaded_packages(&self) -> IsopyLibResult<Vec<Package>> {
        let packages = self.get_available_packages().await?;
        let package_map = packages
            .iter()
            .filter_map(|p| p.asset_path.file_name().map(OsString::from).map(|f| (f, p)))
            .collect::<HashMap<_, _>>();

        let mut packages = Vec::new();

        if self.assets_dir.exists() {
            for result in read_dir(&self.assets_dir).map_err(isopy_lib_other_error)? {
                let entry = result.map_err(isopy_lib_other_error)?;
                let asset_path = entry.path();
                let asset_file_name = entry.file_name();
                let descriptor = package_map
                    .get(&asset_file_name)
                    .and_then(|package| package.descriptor.as_ref())
                    .cloned();
                if let Some(asset_file_name) = asset_file_name.to_str() {
                    if asset_file_name.parse::<AssetMeta>().is_ok() {
                        packages.push(Package {
                            asset_path,
                            descriptor,
                        });
                    }
                }
            }
        }

        Ok(packages)
    }

    async fn download_package(&self, descriptor: &dyn Descriptor) -> IsopyLibResult<PathBuf> {
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
        Ok(asset_path)
    }
}
