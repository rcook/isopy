use crate::error::Result;
use crate::object_model::AssetMeta;
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

    pub fn read_asset_metas(&self) -> Result<Vec<AssetMeta>> {
        let assets_dir = self.dir.join("assets");
        let index_path = assets_dir.join("index.json");
        let index_json = read_to_string(index_path)?;
        let package_records = from_str::<Vec<PackageRecord>>(&index_json)?;

        let mut asset_metas = Vec::new();
        for package_record in package_records {
            for asset_record in package_record.assets {
                if !AssetMeta::definitely_not_an_asset_name(&asset_record.name) {
                    asset_metas.push(AssetMeta::parse(&asset_record.name).expect("Should parse"));
                }
            }
        }
        Ok(asset_metas)
    }
}
