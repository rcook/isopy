use crate::constants::RELEASES_URL;
use crate::object_model::{Asset, AssetMeta, EnvName, LastModified, RepositoryName};
use crate::result::Result;
use crate::serialization::{
    AnonymousEnvRecord, IndexRecord, NamedEnvRecord, PackageRecord, RepositoriesRecord,
    RepositoryRecord, UseRecord,
};
use crate::util::{dir_url, osstr_to_str, path_to_str, safe_write_file};
use md5::compute;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

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

    pub fn read_repositories(&self) -> Result<RepositoriesRecord> {
        let repositories_yaml_path = self.assets_dir.join("repositories.yaml");
        if repositories_yaml_path.is_file() {
            let s = read_to_string(repositories_yaml_path)?;
            Ok(serde_yaml::from_str::<RepositoriesRecord>(&s)?)
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
            Ok(repositories_record)
        }
    }

    pub fn read_index_last_modified(
        &self,
        repository_name: &RepositoryName,
    ) -> Result<Option<LastModified>> {
        let index_yaml_path = self.get_index_yaml_path(repository_name);
        Ok(if index_yaml_path.is_file() {
            let s = read_to_string(&index_yaml_path)?;
            let index_record = serde_yaml::from_str::<IndexRecord>(&s)?;
            Some(index_record.last_modified)
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
        let index_path = self.assets_dir.join("index.json");
        let index_json = read_to_string(index_path)?;
        let package_records = serde_json::from_str::<Vec<PackageRecord>>(&index_json)?;

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

        let s = read_to_string(&named_env_config_path)?;
        let named_env = serde_yaml::from_str::<NamedEnvRecord>(&s)?;
        Ok(Some(named_env))
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

        let s = read_to_string(&anonymous_env_config_path)?;
        let anonymous_env = serde_yaml::from_str::<AnonymousEnvRecord>(&s)?;
        Ok(Some(anonymous_env))
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

        let s = read_to_string(&use_config_path)?;
        let use_ = serde_yaml::from_str::<UseRecord>(&s)?;
        Ok(Some(use_))
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

    fn get_index_yaml_path(&self, repository_name: &RepositoryName) -> PathBuf {
        match repository_name {
            RepositoryName::Default => self.assets_dir.join("index.yaml"),
            RepositoryName::Named(s) => self.assets_dir.join(format!("index-{}.yaml", s)),
        }
    }
}
