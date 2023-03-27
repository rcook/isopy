use crate::error::Result;
use crate::object_model::Asset;
use crate::serialization::PackageRecord;
use serde_json::from_str;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub dir: PathBuf,
}

impl Config {
    pub fn from_dir(dir: PathBuf) -> Self {
        Self { dir: dir }
    }

    pub fn read_assets(&self) -> Result<Vec<Asset>> {
        let assets_dir = self.dir.join("assets");
        let index_path = assets_dir.join("index.json");
        let index_json = read_to_string(index_path)?;
        let packages = from_str::<Vec<PackageRecord>>(&index_json)?;

        let mut assets = Vec::new();
        for package in packages {
            for asset in package.assets {
                if !Asset::definitely_not_an_asset(&asset.name) {
                    assets.push(Asset::from_asset_name(&asset.name).expect("Should parse"));
                }
            }
        }
        Ok(assets)
    }
}
