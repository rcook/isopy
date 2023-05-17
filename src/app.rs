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
use crate::object_model::{Asset, AssetMeta, LastModified, RepositoryName};
use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::serialization::{IndexRec, PackageRec, RepositoriesRec, RepositoryRec};
use crate::util::{dir_url, INDEX_FILE_NAME, RELEASES_URL, REPOSITORIES_FILE_NAME};
use anyhow::Result;
use joat_repo::Repo;
use joatmon::{read_json_file, read_yaml_file, safe_write_file};
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
            read_yaml_file::<RepositoriesRec, _>(repositories_yaml_path)?
        } else {
            let repositories_rec = RepositoriesRec {
                repositories: vec![
                    RepositoryRec::GitHub {
                        name: RepositoryName::parse("default").expect("must parse"),
                        url: dir_url(RELEASES_URL)?,
                        enabled: true,
                    },
                    RepositoryRec::Local {
                        name: RepositoryName::parse("example").expect("must parse"),
                        dir: PathBuf::from("/path/to/local/repository"),
                        enabled: false,
                    },
                ],
            };
            safe_write_file(
                repositories_yaml_path,
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
        let index_yaml_path = self.get_index_yaml_path(repository_name);
        Ok(if index_yaml_path.is_file() {
            Some(read_yaml_file::<IndexRec, _>(&index_yaml_path)?.last_modified)
        } else {
            None
        })
    }

    pub fn write_index_last_modified(
        &self,
        repository_name: &RepositoryName,
        last_modified: &LastModified,
    ) -> Result<()> {
        let index_yaml_path = self.get_index_yaml_path(repository_name);
        safe_write_file(
            index_yaml_path,
            serde_yaml::to_string(&IndexRec {
                last_modified: last_modified.clone(),
            })?,
            true,
        )?;
        Ok(())
    }

    pub fn get_index_json_path(&self, repository_name: &RepositoryName) -> PathBuf {
        match repository_name {
            RepositoryName::Default => self.repo.shared_dir().join(INDEX_FILE_NAME),
            RepositoryName::Named(s) => self.repo.shared_dir().join(format!("index-{}.json", s)),
        }
    }

    pub fn make_asset_path(&self, asset: &Asset) -> PathBuf {
        self.repo.shared_dir().join(&asset.name)
    }

    pub fn read_assets(&self) -> Result<Vec<Asset>> {
        let index_json_path = self.repo.shared_dir().join(INDEX_FILE_NAME);
        let package_recs = read_json_file::<Vec<PackageRec>, _>(&index_json_path)?;

        let mut assets = Vec::new();
        for package_rec in package_recs {
            for asset_rec in package_rec.assets {
                if !AssetMeta::definitely_not_an_asset_name(&asset_rec.name) {
                    let meta = AssetMeta::parse(&asset_rec.name)?;
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

    fn make_repository(rec: RepositoryRec) -> Result<(RepositoryName, bool, Box<dyn Repository>)> {
        Ok(match rec {
            RepositoryRec::GitHub { name, url, enabled } => {
                (name, enabled, Box::new(GitHubRepository::new(url)?))
            }
            RepositoryRec::Local { name, dir, enabled } => {
                (name, enabled, Box::new(LocalRepository::new(dir)))
            }
        })
    }

    fn get_index_yaml_path(&self, repository_name: &RepositoryName) -> PathBuf {
        match repository_name {
            RepositoryName::Default => self.repo.shared_dir().join("index.yaml"),
            RepositoryName::Named(s) => self.repo.shared_dir().join(format!("index-{}.yaml", s)),
        }
    }
}
