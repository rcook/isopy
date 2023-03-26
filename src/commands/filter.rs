use crate::config::Config;
use crate::error::Result;
use crate::object_model::{AssetFilter, AssetInfo, Tag};
use crate::serialization::Package;
use crate::version::Version;
use serde_json::from_str;
use std::fs::read_to_string;

pub fn do_filter(config: &Config) -> Result<()> {
    let assets_dir = config.dir.join("assets");
    let index_path = assets_dir.join("index.json");
    let index_json = read_to_string(index_path)?;
    let packages = from_str::<Vec<Package>>(&index_json)?;

    let mut asset_infos = Vec::new();
    for package in packages {
        for asset in package.assets {
            if !AssetInfo::definitely_not_an_asset(&asset.name) {
                asset_infos.push(AssetInfo::from_asset_name(&asset.name).expect("Should parse"));
            }
        }
    }
    println!("count={}", asset_infos.len());

    let mut asset_filter = AssetFilter::default_for_platform();
    asset_filter.version = Some(Version::new(3, 11, 1));
    asset_filter.tag = Some(Tag::NewStyle(String::from("20230116")));

    let filtered_asset_infos = asset_filter.filter(asset_infos.iter().map(|x| x).into_iter());
    println!("filtered_count={}", filtered_asset_infos.len());
    for a in filtered_asset_infos {
        println!("a={:?}", a)
    }

    Ok(())
}
