use crate::error::Result;
use crate::object_model::AssetName;
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

    pub fn read_asset_names(&self) -> Result<Vec<AssetName>> {
        let assets_dir = self.dir.join("assets");
        let index_path = assets_dir.join("index.json");
        let index_json = read_to_string(index_path)?;
        let packages = from_str::<Vec<PackageRecord>>(&index_json)?;

        let mut asset_names = Vec::new();
        for package in packages {
            for asset in package.assets {
                if !AssetName::definitely_not_an_asset_name(&asset.name) {
                    asset_names.push(AssetName::parse(&asset.name).expect("Should parse"));
                }
            }
        }
        Ok(asset_names)
    }
}
