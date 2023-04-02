use crate::constants::RELEASES_URL;
use crate::object_model::{Asset, AssetMeta, EnvName, LastModified, RepositoryName};
use crate::repository::{GitHubRepository, LocalRepository, Repository as RepositoryTrait};
use crate::result::Result;
use crate::serialization::{
    AnonymousEnvRecord, IndexRecord, NamedEnvRecord, PackageRecord, RepositoriesRecord,
    RepositoryRecord, UseRecord,
};
use crate::util::{
    dir_url, osstr_to_str, path_to_str, read_json_file, read_yaml_file, safe_write_file,
};
use md5::compute;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct Repository {
    pub name: RepositoryName,
    pub repository: Box<dyn RepositoryTrait>,
}

#[derive(Debug)]
pub struct App {
    pub cwd: PathBuf,
    pub dir: PathBuf,
    pub assets_dir: PathBuf,
    pub named_envs_dir: PathBuf,
    pub anonymous_envs_dir: PathBuf,
    pub uses_dir: PathBuf,
}

impl App {
    pub fn new(cwd: PathBuf, dir: PathBuf) -> Self {
        let assets_dir = dir.join("assets");
        let named_envs_dir = dir.join("envs");
        let anonymous_envs_dir = dir.join("hashed");
        let uses_dir = dir.join("uses");
        Self {
            cwd: cwd,
            dir: dir,
            assets_dir: assets_dir,
            named_envs_dir: named_envs_dir,
            anonymous_envs_dir: anonymous_envs_dir,
            uses_dir: uses_dir,
        }
    }

    pub fn read_repositories(&self) -> Result<Vec<Repository>> {
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
            .map(|x| Repository {
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
            &index_yaml_path,
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
                        meta: meta,
                    });
                }
            }
        }
        Ok(assets)
    }

    pub fn named_env_dir(&self, env_name: &EnvName) -> PathBuf {
        self.named_envs_dir.join(env_name.as_str())
    }

    pub fn read_named_envs(&self) -> Result<Vec<NamedEnvRecord>> {
        let mut named_envs = Vec::new();
        for d in read_dir(&self.named_envs_dir)? {
            let env_name = match EnvName::parse(osstr_to_str(&d?.file_name())?) {
                Some(x) => x,
                None => continue,
            };

            let named_env = match self.read_named_env(&env_name)? {
                Some(x) => x,
                None => continue,
            };

            named_envs.push(named_env)
        }

        Ok(named_envs)
    }

    pub fn read_named_env(&self, env_name: &EnvName) -> Result<Option<NamedEnvRecord>> {
        let named_env_config_path = self.named_env_dir(env_name).join("env.yaml");
        if !named_env_config_path.is_file() {
            return Ok(None);
        }

        Ok(Some(read_yaml_file::<NamedEnvRecord, _>(
            &named_env_config_path,
        )?))
    }

    pub fn anonymous_env_dir<P>(&self, project_config_path: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let digest = compute(path_to_str(project_config_path.as_ref())?);
        let hex_digest = format!("{:x}", digest);
        Ok(self.anonymous_envs_dir.join(&hex_digest))
    }

    pub fn read_anonymous_env<S>(&self, hex_digest: S) -> Result<Option<AnonymousEnvRecord>>
    where
        S: AsRef<str>,
    {
        let anonymous_env_config_path = self
            .anonymous_envs_dir
            .join(hex_digest.as_ref())
            .join("env.yaml");
        if !anonymous_env_config_path.is_file() {
            return Ok(None);
        }

        Ok(Some(read_yaml_file::<AnonymousEnvRecord, _>(
            &anonymous_env_config_path,
        )?))
    }

    pub fn read_anonymous_envs(&self) -> Result<Vec<AnonymousEnvRecord>> {
        let mut anonymous_envs = Vec::new();

        if self.anonymous_envs_dir.is_dir() {
            for d in read_dir(&self.anonymous_envs_dir)? {
                let file_name = d?.file_name();
                let hex_digest = osstr_to_str(&file_name)?;

                let anonymous_env = match self.read_anonymous_env(&hex_digest)? {
                    Some(x) => x,
                    None => continue,
                };

                anonymous_envs.push(anonymous_env)
            }
        }

        Ok(anonymous_envs)
    }

    pub fn use_dir<P>(&self, dir: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let digest = compute(path_to_str(dir.as_ref())?);
        let hex_digest = format!("{:x}", digest);
        Ok(self.uses_dir.join(&hex_digest))
    }

    pub fn read_use<S>(&self, hex_digest: S) -> Result<Option<UseRecord>>
    where
        S: AsRef<str>,
    {
        let use_config_path = self.uses_dir.join(hex_digest.as_ref()).join("use.yaml");
        if !use_config_path.is_file() {
            return Ok(None);
        }

        Ok(Some(read_yaml_file::<UseRecord, _>(&use_config_path)?))
    }

    pub fn read_uses(&self) -> Result<Vec<UseRecord>> {
        let mut uses = Vec::new();

        if self.uses_dir.is_dir() {
            for d in read_dir(&self.uses_dir)? {
                let file_name = d?.file_name();
                let hex_digest = osstr_to_str(&file_name)?;

                let use_ = match self.read_use(&hex_digest)? {
                    Some(x) => x,
                    None => continue,
                };

                uses.push(use_)
            }
        }

        Ok(uses)
    }

    fn make_repository(
        record: RepositoryRecord,
    ) -> Result<(RepositoryName, bool, Box<dyn RepositoryTrait>)> {
        Ok(match record {
            RepositoryRecord::GitHub { name, url, enabled } => {
                (name, enabled, Box::new(GitHubRepository::new(url.clone())?))
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
