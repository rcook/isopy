use crate::error::Result;
use crate::object_model::{Asset, AssetMeta};
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
        let package_records = from_str::<Vec<PackageRecord>>(&index_json)?;

        let mut assets = Vec::new();
        for package_record in package_records {
            for asset_record in package_record.assets {
                if !AssetMeta::definitely_not_an_asset_name(&asset_record.name) {
                    let meta = AssetMeta::parse(&asset_record.name).expect("Should parse");
                    assets.push(Asset {
                        name: asset_record.name,
                        url: asset_record.url,
                        size: asset_record.size,
                        meta: meta,
                    });
                }
            }
        }
        Ok(assets)
    }
}
