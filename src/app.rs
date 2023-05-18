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
use crate::asset::{download_asset, get_asset};
use crate::cli::PythonVersion;
use crate::constants::{
    ENV_FILE_NAME, INDEX_FILE_NAME, RELEASES_FILE_NAME, RELEASES_URL, REPOSITORIES_FILE_NAME,
};
use crate::object_model::{Asset, AssetMeta, LastModified, RepositoryName};
use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::serialization::EnvRec;
use crate::serialization::{IndexRec, PackageRec, RepositoriesRec, RepositoryRec};
use crate::unpack::unpack_file;
use crate::url::dir_url;
use anyhow::{bail, Result};
use joat_repo::Repo;
use joat_repo::{DirInfo, Link};
use joatmon::{label_file_name, read_json_file, read_yaml_file, safe_write_file};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

pub struct RepositoryInfo {
    pub name: RepositoryName,
    pub repository: Box<dyn Repository>,
}

#[derive(Debug)]
pub struct App {
    pub cwd: PathBuf,
    pub repo: Repo,
}

impl App {
    pub fn new(cwd: PathBuf, repo: Repo) -> Self {
        Self { cwd, repo }
    }

    pub fn read_repositories(&self) -> Result<Vec<RepositoryInfo>> {
        let repositories_yaml_path = self.repo.shared_dir().join(REPOSITORIES_FILE_NAME);
        let repositories_rec = if repositories_yaml_path.is_file() {
            read_yaml_file::<RepositoriesRec>(&repositories_yaml_path)?
        } else {
            let repositories_rec = RepositoriesRec {
                repositories: vec![
                    RepositoryRec::GitHub {
                        name: RepositoryName::Default,
                        url: dir_url(&RELEASES_URL)?,
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
            .map(Self::make_repository)
            .collect::<Result<Vec<_>>>()?;
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

    pub fn read_index_last_modified(
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

    pub fn write_index_last_modified(
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

    pub fn releases_path(&self, repository_name: &RepositoryName) -> PathBuf {
        if repository_name.is_default() {
            self.repo.shared_dir().join(RELEASES_FILE_NAME)
        } else {
            label_file_name(
                &self.repo.shared_dir().join(RELEASES_FILE_NAME),
                repository_name.as_str(),
            )
            .expect("must be valid")
        }
    }

    pub fn make_asset_path(&self, asset: &Asset) -> PathBuf {
        self.repo.shared_dir().join(&asset.name)
    }

    pub fn read_assets(&self) -> Result<Vec<Asset>> {
        let index_json_path = self.repo.shared_dir().join(RELEASES_FILE_NAME);
        let package_recs = read_json_file::<Vec<PackageRec>>(&index_json_path)?;

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

    pub async fn init_project(&self, python_version: &PythonVersion) -> Result<()> {
        let assets = self.read_assets()?;
        let asset = get_asset(&assets, python_version)?;

        let mut asset_path = self.make_asset_path(asset);
        if !asset_path.is_file() {
            asset_path = download_asset(self, asset).await?;
        }

        let Some(dir_info) = self.repo.init(&self.cwd)? else {
            bail!(
                "Could not initialize metadirectory for directory {}",
                self.cwd.display()
            )
        };

        unpack_file(&asset_path, dir_info.data_dir())?;

        safe_write_file(
            &dir_info.data_dir().join(ENV_FILE_NAME),
            serde_yaml::to_string(&EnvRec {
                config_path: self.cwd.clone(),
                python_dir_rel: PathBuf::from("python"),
                version: asset.meta.version.clone(),
                tag: asset.tag.clone(),
            })?,
            false,
        )?;

        Ok(())
    }

    pub fn find_dir_info(&self, cwd: &Path) -> Result<Option<DirInfo>> {
        let Some(link) = self.find_link(cwd)? else {
            return Ok(None)
        };

        let Some(dir_info) = self.repo.get(link.project_dir())? else {
            return Ok(None)
        };

        Ok(Some(dir_info))
    }

    fn make_repository(rec: RepositoryRec) -> Result<(RepositoryName, bool, Box<dyn Repository>)> {
        Ok(match rec {
            RepositoryRec::GitHub { name, url, enabled } => {
                (name, enabled, Box::new(GitHubRepository::new(&url)?))
            }
            RepositoryRec::Local { name, dir, enabled } => {
                (name, enabled, Box::new(LocalRepository::new(dir)))
            }
        })
    }

    fn index_path(&self, repository_name: &RepositoryName) -> PathBuf {
        if repository_name.is_default() {
            self.repo.shared_dir().join(INDEX_FILE_NAME)
        } else {
            label_file_name(
                &self.repo.shared_dir().join(INDEX_FILE_NAME),
                repository_name.as_str(),
            )
            .expect("must be valid")
        }
    }

    fn find_link(&self, dir: &Path) -> Result<Option<Link>> {
        let mut map = self
            .repo
            .list_links()?
            .into_iter()
            .map(|x| (x.project_dir().to_path_buf(), x))
            .collect::<HashMap<_, _>>();

        let mut d = dir;
        loop {
            if let Some(link) = map.remove(d) {
                return Ok(Some(link));
            }

            if let Some(p) = d.parent() {
                d = p;
            } else {
                return Ok(None);
            }
        }
    }
}
