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
use crate::result::Result;
use crate::serialization::{
    IndexRecord, NamedEnvironmentRecord, PackageRecord, ProjectEnvironmentRecord, ProjectRecord,
    RepositoriesRecord, RepositoryRecord, UseRecord,
};
use crate::util::{
    dir_url, find_project_config_path, osstr_to_str, read_json_file, read_yaml_file,
    safe_write_file, HexDigest, RELEASES_URL,
};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct RepositoryInfo {
    pub name: RepositoryName,
    pub repository: Box<dyn Repository>,
}

#[derive(Debug)]
pub struct App {
    pub cwd: PathBuf,
    pub dir: PathBuf,
    pub assets_dir: PathBuf,
    named_environments_dir: PathBuf,
    project_environments_dir: PathBuf,
    uses_dir: PathBuf,
}

impl App {
    pub fn new(cwd: PathBuf, dir: PathBuf) -> Self {
        let assets_dir = dir.join("assets");
        let named_environments_dir = dir.join("envs");
        let project_environments_dir = dir.join("hashed");
        let uses_dir = dir.join("uses");
        Self {
            cwd,
            dir,
            assets_dir,
            named_environments_dir,
            project_environments_dir,
            uses_dir,
        }
    }

    pub fn read_repositories(&self) -> Result<Vec<RepositoryInfo>> {
        let repositories_yaml_path = self.assets_dir.join("repositories.yaml");
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
            RepositoryName::Default => self.assets_dir.join("index.json"),
            RepositoryName::Named(s) => self.assets_dir.join(format!("index-{}.json", s)),
        }
    }

    pub fn make_asset_path(&self, asset: &Asset) -> PathBuf {
        self.assets_dir.join(&asset.name)
    }

    pub fn read_assets(&self) -> Result<Vec<Asset>> {
        let index_json_path = self.assets_dir.join("index.json");
        let package_records = read_json_file::<Vec<PackageRecord>, _>(&index_json_path)?;

        let mut assets = Vec::new();
        for package_record in package_records {
            for asset_record in package_record.assets {
                if !AssetMeta::definitely_not_an_asset_name(&asset_record.name) {
                    let meta = AssetMeta::parse(&asset_record.name).expect("Should parse");
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

    pub fn named_environment_dir(&self, name: &EnvironmentName) -> PathBuf {
        self.named_environments_dir.join(name.as_str())
    }

    pub fn read_named_environments(&self) -> Result<Vec<NamedEnvironmentRecord>> {
        let mut recs = Vec::new();
        for d in read_dir(&self.named_environments_dir)? {
            let name = match EnvironmentName::parse(osstr_to_str(&d?.file_name())?) {
                Some(x) => x,
                None => continue,
            };

            let rec = match self.read_named_environment(&name)? {
                Some(x) => x,
                None => continue,
            };

            recs.push(rec)
        }

        Ok(recs)
    }

    pub fn read_named_environment(
        &self,
        name: &EnvironmentName,
    ) -> Result<Option<NamedEnvironmentRecord>> {
        let config_path = self.named_environment_dir(name).join("env.yaml");
        if !config_path.is_file() {
            return Ok(None);
        }

        Ok(Some(read_yaml_file::<NamedEnvironmentRecord, _>(
            &config_path,
        )?))
    }

    pub fn project_environment_dir<P>(&self, config_path: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let hex_digest = HexDigest::from_path(config_path)?;
        Ok(self.project_environments_dir.join(hex_digest.as_str()))
    }

    pub fn read_project_environment<S>(
        &self,
        hex_digest: S,
    ) -> Result<Option<ProjectEnvironmentRecord>>
    where
        S: AsRef<str>,
    {
        let config_path = self
            .project_environments_dir
            .join(hex_digest.as_ref())
            .join("env.yaml");
        if !config_path.is_file() {
            return Ok(None);
        }

        Ok(Some(read_yaml_file::<ProjectEnvironmentRecord, _>(
            &config_path,
        )?))
    }

    pub fn read_project_environments(&self) -> Result<Vec<ProjectEnvironmentRecord>> {
        let mut recs = Vec::new();

        if self.project_environments_dir.is_dir() {
            for d in read_dir(&self.project_environments_dir)? {
                let file_name = d?.file_name();
                let hex_digest = osstr_to_str(&file_name)?;

                let rec = match self.read_project_environment(hex_digest)? {
                    Some(x) => x,
                    None => continue,
                };

                recs.push(rec)
            }
        }

        Ok(recs)
    }

    pub fn use_dir<P>(&self, dir: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let hex_digest = HexDigest::from_path(dir)?;
        Ok(self.uses_dir.join(hex_digest.as_str()))
    }

    pub fn read_use<S>(&self, hex_digest: S) -> Result<Option<UseRecord>>
    where
        S: AsRef<str>,
    {
        let config_path = self.uses_dir.join(hex_digest.as_ref()).join("use.yaml");
        if !config_path.is_file() {
            return Ok(None);
        }

        Ok(Some(read_yaml_file::<UseRecord, _>(&config_path)?))
    }

    pub fn read_uses(&self) -> Result<Vec<UseRecord>> {
        let mut recs = Vec::new();

        if self.uses_dir.is_dir() {
            for d in read_dir(&self.uses_dir)? {
                let file_name = d?.file_name();
                let hex_digest = osstr_to_str(&file_name)?;

                let rec = match self.read_use(hex_digest)? {
                    Some(x) => x,
                    None => continue,
                };

                recs.push(rec)
            }
        }

        Ok(recs)
    }

    pub fn read_project<P>(&self, start_dir: P) -> Result<Option<Project>>
    where
        P: Into<PathBuf>,
    {
        Ok(match find_project_config_path(start_dir)? {
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
            RepositoryName::Default => self.assets_dir.join("index.yaml"),
            RepositoryName::Named(s) => self.assets_dir.join(format!("index-{}.yaml", s)),
        }
    }
}
