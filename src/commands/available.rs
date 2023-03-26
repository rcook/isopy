use crate::config::Config;
use crate::error::Result;
use crate::object_model::AssetInfo;
use std::fs::read_dir;

pub fn do_available(config: &Config) -> Result<()> {
    let assets_dir = config.dir.join("assets");
    for e in read_dir(assets_dir)? {
        let temp = e?;
        if let Some(asset_info) =
            AssetInfo::from_asset_name(temp.file_name().to_str().expect("Must be a valid string"))
        {
            println!("asset_info={:?}", asset_info)
        }
    }
    Ok(())
}
