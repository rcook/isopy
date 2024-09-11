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
use crate::serialization::{
    Index, Package as Package_serialization, Repositories, Repository as Repository_serialization,
};
use crate::tag::Tag;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use isopy_lib::{dir_url, download_stream, IsopyLibResult, LastModified, Package, Plugin};
use joatmon::label_file_name;
use joatmon::read_yaml_file;
use joatmon::{read_json_file, safe_write_file};
use std::cmp::Ordering;
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
        let packages = read_json_file::<Vec<Package_serialization>>(&index_path)?;

        let mut assets = Vec::new();
        for package in packages {
            for asset in package.assets {
                if !AssetMeta::definitely_not_an_asset_name(&asset.name) {
                    let meta = asset.name.parse::<AssetMeta>()?;
                    assets.push(Asset {
                        name: asset.name,
                        tag: package.tag.clone(),
                        url: asset.url,
                        size: asset.size,
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
            rec: Repository_serialization,
        ) -> (RepositoryName, bool, Box<dyn Repository>) {
            match rec {
                Repository_serialization::GitHub { name, url, enabled } => (
                    name,
                    enabled,
                    Box::new(GitHubRepository::new(offline, &url)),
                ),
                Repository_serialization::Local { name, dir, enabled } => {
                    (name, enabled, Box::new(LocalRepository::new(&dir)))
                }
            }
        }

        let repositories_yaml_path = self.dir.join(REPOSITORIES_FILE_NAME);
        let repositories = if repositories_yaml_path.is_file() {
            read_yaml_file::<Repositories>(&repositories_yaml_path)?
        } else {
            let repositories = Repositories {
                repositories: vec![
                    Repository_serialization::GitHub {
                        name: RepositoryName::Default,
                        url: dir_url(&RELEASES_URL),
                        enabled: true,
                    },
                    Repository_serialization::Local {
                        name: RepositoryName::Example,
                        dir: PathBuf::from("/path/to/local/repository"),
                        enabled: false,
                    },
                ],
            };
            safe_write_file(
                &repositories_yaml_path,
                serde_yaml::to_string(&repositories)?,
                false,
            )?;
            repositories
        };

        let all_repositories = repositories
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
            Some(read_yaml_file::<Index>(&index_path)?.last_modified)
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
            serde_yaml::to_string(&Index {
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
}
