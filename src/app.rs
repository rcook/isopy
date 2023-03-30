use crate::error::Result;
use crate::object_model::{Asset, AssetMeta, EnvName};
use crate::serialization::{EnvRecord, PackageRecord};
use crate::util::osstr_to_str;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

#[derive(Debug)]
pub struct App {
    pub cwd: PathBuf,
    pub dir: PathBuf,
    pub assets_dir: PathBuf,
    pub envs_dir: PathBuf,
    pub hashed_dir: PathBuf,
    pub uses_dir: PathBuf,
}

impl App {
    pub fn new(cwd: PathBuf, dir: PathBuf) -> Self {
        let assets_dir = dir.join("assets");
        let envs_dir = dir.join("envs");
        let hashed_dir = dir.join("hashed");
        let uses_dir = dir.join("uses");
        Self {
            cwd: cwd,
            dir: dir,
            assets_dir: assets_dir,
            envs_dir: envs_dir,
            hashed_dir: hashed_dir,
            uses_dir: uses_dir,
        }
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

    pub fn env_dir(&self, env_name: &EnvName) -> PathBuf {
        self.envs_dir.join(env_name.as_str())
    }

    pub fn read_envs(&self) -> Result<Vec<EnvRecord>> {
        let mut envs = Vec::new();
        for d in read_dir(&self.envs_dir)? {
            let env_name = match EnvName::parse(osstr_to_str(&d?.file_name())?) {
                Some(x) => x,
                None => continue,
            };

            let env = match self.read_env(&env_name)? {
                Some(x) => x,
                None => continue,
            };

            envs.push(env)
        }

        Ok(envs)
    }

    pub fn read_env(&self, env_name: &EnvName) -> Result<Option<EnvRecord>> {
        let env_config_path = self.envs_dir.join(env_name.as_str()).join("env.yaml");
        if !env_config_path.is_file() {
            return Ok(None);
        }

        let s = read_to_string(&env_config_path)?;
        let env = serde_yaml::from_str::<EnvRecord>(&s)?;
        Ok(Some(env))
    }
}