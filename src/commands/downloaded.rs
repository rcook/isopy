use crate::config::Config;
use crate::error::Result;
use crate::object_model::AssetName;
use std::fs::read_dir;

pub fn do_downloaded(config: &Config) -> Result<()> {
    let assets_dir = config.dir.join("assets");
    for e in read_dir(assets_dir)? {
        let temp = e?;
        if let Some(asset_name) =
            AssetName::parse(temp.file_name().to_str().expect("Must be a valid string"))
        {
            println!("asset_name={:?}", asset_name)
        }
    }
    Ok(())
}
