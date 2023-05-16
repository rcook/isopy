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
use crate::object_model::{
    Asset, AssetMeta, EnvironmentName, LastModified, Project, RepositoryName,
};
use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::serialization::{
    IndexRecord, NamedEnvironmentRecord, PackageRecord, ProjectEnvironmentRecord, ProjectRecord,
    RepositoriesRecord, RepositoryRecord,
};
use crate::util::{dir_url, find_project_config_path, osstr_to_str, HexDigest, RELEASES_URL};
use anyhow::{bail, Result};
use joat_repo::{DirInfo, Link, MetaId, Repo};
use joatmon::{read_json_file, read_yaml_file, safe_write_file};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

const REPOSITORIES_FILE_NAME: &str = "repositories.yaml";
const INDEX_FILE_NAME: &str = "index.json";

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
    pub fn new(cwd: PathBuf, dir: PathBuf, repo: Repo) -> Self {
        Self { cwd, repo }
    }

    pub fn read_repositories(&self) -> Result<Vec<RepositoryInfo>> {
        let repositories_yaml_path = self.repo.shared_dir().join(REPOSITORIES_FILE_NAME);
        let repositories_record = if repositories_yaml_path.is_file() {
            read_yaml_file::<RepositoriesRecord, _>(repositories_yaml_path)?
        } else {
            let repositories_record = RepositoriesRecord {
                repositories: vec![
                    RepositoryRecord::GitHub {
                        name: RepositoryName::parse("default").expect("must parse"),
                        url: dir_url(RELEASES_URL)?,
                        enabled: true,
                    },
                    RepositoryRecord::Local {
                        name: RepositoryName::parse("example").expect("must parse"),
                        dir: PathBuf::from("/path/to/local/repository"),
                        enabled: false,
                    },
                ],
            };
            safe_write_file(
                repositories_yaml_path,
                serde_yaml::to_string(&repositories_record)?,
                false,
            )?;
            repositories_record
        };

        let all_repositories = repositories_record
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
            Some(read_yaml_file::<IndexRecord, _>(&index_yaml_path)?.last_modified)
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
            serde_yaml::to_string(&IndexRecord {
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
        let package_records = read_json_file::<Vec<PackageRecord>, _>(&index_json_path)?;

        let mut assets = Vec::new();
        for package_record in package_records {
            for asset_record in package_record.assets {
                if !AssetMeta::definitely_not_an_asset_name(&asset_record.name) {
                    let meta = AssetMeta::parse(&asset_record.name)?;
                    assets.push(Asset {
                        name: asset_record.name,
                        tag: package_record.tag.clone(),
                        url: asset_record.url,
                        size: asset_record.size,
                        meta,
                    });
                }
            }
        }
        Ok(assets)
    }

    pub fn read_project<P>(&self, start_dir: P) -> Result<Option<Project>>
    where
        P: AsRef<Path>,
    {
        Ok(match find_project_config_path(start_dir) {
            None => None,
            Some(p) => {
                let project_record = read_yaml_file::<ProjectRecord, _>(&p)?;
                Some(Project {
                    config_path: p,
                    python_version: project_record.python_version,
                    tag: project_record.tag,
                })
            }
        })
    }

    fn make_repository(
        record: RepositoryRecord,
    ) -> Result<(RepositoryName, bool, Box<dyn Repository>)> {
        Ok(match record {
            RepositoryRecord::GitHub { name, url, enabled } => {
                (name, enabled, Box::new(GitHubRepository::new(url)?))
            }
            RepositoryRecord::Local { name, dir, enabled } => {
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
